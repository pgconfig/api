package rules

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	"github.com/pgconfig/api/pkg/version"
)

// TuningRequest is the canonical, explicit input to the tuning operation.
type TuningRequest struct {
	OS                string          `json:"os"`
	Arch              string          `json:"arch"`
	TotalRAM          bytes.Byte      `json:"total_ram"`
	Profile           profile.Profile `json:"profile"`
	DiskType          string          `json:"disk_type"`
	MaxConnections    int             `json:"max_connections"`
	TotalCPU          int             `json:"total_cpu"`
	PostgreSQLVersion string          `json:"postgres_version"`
}

// TuningRecommendation describes a final PostgreSQL setting and why it has
// that value.
type TuningRecommendation struct {
	Value  string `json:"value"`
	Reason string `json:"reason"`
}

// TuningResult is the provenance-aware result returned by Tune.
type TuningResult struct {
	Request            TuningRequest                   `json:"request"`
	Recommendations    map[string]TuningRecommendation `json:"recommendations"`
	ApplicationVersion string                          `json:"application_version"`
	legacy             *category.ExportCfg
}

// NewTuningRequest converts a legacy input into the canonical request model.
func NewTuningRequest(in input.Input) TuningRequest {
	return TuningRequest{
		OS:                in.OS,
		Arch:              in.Arch,
		TotalRAM:          in.TotalRAM,
		Profile:           in.Profile,
		DiskType:          in.DiskType,
		MaxConnections:    in.MaxConnections,
		TotalCPU:          in.TotalCPU,
		PostgreSQLVersion: strconv.FormatFloat(float64(in.PostgresVersion), 'f', -1, 32),
	}
}

func (r TuningRequest) legacyInput(rulesVersion float32) input.Input {
	return input.Input{
		OS:              r.OS,
		Arch:            r.Arch,
		TotalRAM:        r.TotalRAM,
		Profile:         r.Profile,
		DiskType:        r.DiskType,
		MaxConnections:  r.MaxConnections,
		TotalCPU:        r.TotalCPU,
		PostgresVersion: rulesVersion,
	}
}

// Tune produces rich recommendations through the shared tuning pipeline.
func Tune(request TuningRequest) (*TuningResult, error) {
	normalized := normalizeRequest(request)
	rulesVersion, err := parseRulesVersion(normalized.PostgreSQLVersion)
	if err != nil {
		return nil, err
	}
	calculationRequest := normalized.legacyInput(rulesVersion)
	legacy, adjustments, err := computeWithAdjustments(calculationRequest)
	if err != nil {
		return nil, err
	}

	recommendations := make(map[string]TuningRecommendation)
	for _, group := range legacy.ToSlice(rulesVersion, false, "") {
		for _, parameter := range group.Parameters {
			if parameter.Name == "listen_addresses" {
				continue
			}
			recommendations[parameter.Name] = TuningRecommendation{
				Value:  parameter.Value,
				Reason: recommendationReason(parameter.Name, parameter.Value, normalized, adjustments[parameter.Name]),
			}
		}
	}

	return &TuningResult{
		Request:            normalized,
		Recommendations:    recommendations,
		ApplicationVersion: version.Pretty(),
		legacy:             legacy,
	}, nil
}

// TuneCompatibility uses the shared tuning operation while retaining the
// historical projection and errors expected by legacy consumers.
func TuneCompatibility(legacyRequest input.Input) (*TuningResult, error) {
	canonicalRequest := NewTuningRequest(legacyRequest)
	result, tuningErr := Tune(canonicalRequest)
	compatibility, _, compatibilityErr := computeWithAdjustments(legacyRequest)
	if compatibilityErr != nil {
		return nil, compatibilityErr
	}
	if tuningErr != nil {
		return &TuningResult{
			Request:            normalizeRequest(canonicalRequest),
			Recommendations:    map[string]TuningRecommendation{},
			ApplicationVersion: version.Pretty(),
			legacy:             compatibility,
		}, nil
	}
	result.legacy = compatibility
	return result, nil
}

// CompatibilityProjection returns the legacy category model used by REST v1
// and the CLI while those consumers are migrated.
func (r *TuningResult) CompatibilityProjection() *category.ExportCfg {
	return r.legacy
}

func normalizeRequest(request TuningRequest) TuningRequest {
	request.OS = strings.ToLower(strings.TrimSpace(request.OS))
	request.Arch = strings.ToLower(strings.TrimSpace(request.Arch))
	switch request.Arch {
	case "i686":
		request.Arch = "386"
	case "x86-64":
		request.Arch = "amd64"
	}
	request.DiskType = strings.ToUpper(strings.TrimSpace(request.DiskType))
	request.PostgreSQLVersion = strings.TrimSpace(request.PostgreSQLVersion)
	return request
}

func parseRulesVersion(postgresVersion string) (float32, error) {
	parts := strings.Split(strings.TrimSpace(postgresVersion), ".")
	major, err := strconv.Atoi(parts[0])
	if err != nil || major < 1 {
		return 0, fmt.Errorf("invalid PostgreSQL Version %q", postgresVersion)
	}
	if major >= 10 {
		return float32(major), nil
	}
	if len(parts) < 2 {
		return 0, fmt.Errorf("invalid PostgreSQL Version %q", postgresVersion)
	}
	minor, err := strconv.Atoi(parts[1])
	if err != nil {
		return 0, fmt.Errorf("invalid PostgreSQL Version %q", postgresVersion)
	}
	parsed, err := strconv.ParseFloat(fmt.Sprintf("%d.%d", major, minor), 32)
	if err != nil {
		return 0, fmt.Errorf("invalid PostgreSQL Version %q", postgresVersion)
	}
	return float32(parsed), nil
}

func recommendationReason(name, value string, request TuningRequest, adjustments []ruleAdjustment) string {
	switch name {
	case "shared_buffers":
		return sharedBuffersReason(value, request, adjustments)
	case "work_mem":
		if reason, capped := memoryCapReason(value, adjustments); capped {
			return reason
		}
		return fmt.Sprintf("Set to %s from available memory for the %s profile and %d connections.", value, request.Profile, request.MaxConnections)
	case "maintenance_work_mem":
		if reason, capped := memoryCapReason(value, adjustments); capped {
			return reason
		}
		return fmt.Sprintf("Set to %s as 5%% of memory available to the %s profile.", value, request.Profile)
	case "effective_cache_size":
		return fmt.Sprintf("Set to %s from memory available to the PostgreSQL and operating-system caches.", value)
	case "max_connections":
		return fmt.Sprintf("Set to %s to match the requested connection limit.", value)
	case "random_page_cost":
		return fmt.Sprintf("Set to %s for %s storage and the %s workload profile.", value, request.DiskType, request.Profile)
	case "effective_io_concurrency":
		if request.OS == Windows {
			return fmt.Sprintf("Set to %s for the requested %s storage; the storage policy replaces the initial Windows compatibility value.", value, request.DiskType)
		}
		return fmt.Sprintf("Set to %s for the requested %s storage.", value, request.DiskType)
	case "maintenance_io_concurrency":
		return fmt.Sprintf("Set to %s for the requested %s storage.", value, request.DiskType)
	case "io_workers":
		return ioWorkersReason(value, request)
	case "io_max_combine_limit", "io_max_concurrency":
		return fmt.Sprintf("Set to %s for the %s workload profile.", value, request.Profile)
	case "max_worker_processes", "max_parallel_workers":
		if request.TotalCPU < 8 {
			return fmt.Sprintf("Set to %s by applying the minimum of 8 to %d logical CPUs.", value, request.TotalCPU)
		}
		return fmt.Sprintf("Set to %s to match the %d logical CPUs.", value, request.TotalCPU)
	case "max_parallel_workers_per_gather":
		if request.Profile != profile.DW {
			return fmt.Sprintf("Set to %s as the parallel-query default for the %s workload profile.", value, request.Profile)
		}
		halfCPU := request.TotalCPU / 2
		if halfCPU < 2 {
			return fmt.Sprintf("Set to %s from half of %d logical CPUs, raised to the minimum of 2 for the DW workload profile.", value, request.TotalCPU)
		}
		return fmt.Sprintf("Set to %s from half of %d logical CPUs for the DW workload profile.", value, request.TotalCPU)
	case "min_wal_size", "max_wal_size":
		return fmt.Sprintf("Set to %s for the %s workload profile.", value, request.Profile)
	case "wal_buffers":
		if request.Profile == profile.DW {
			return fmt.Sprintf("Set to %s for write-heavy DW workloads.", value)
		}
		if request.Profile == profile.OLTP && value == "32MB" {
			return fmt.Sprintf("Set to %s for OLTP because derived shared_buffers exceeds 8GB.", value)
		}
		return fmt.Sprintf("Set to %s to use automatic PostgreSQL tuning for the %s workload profile.", value, request.Profile)
	case "checkpoint_completion_target":
		return fmt.Sprintf("Set to %s to spread checkpoint I/O across most of the checkpoint interval.", value)
	case "checkpoint_segments":
		return fmt.Sprintf("Set to %s as the legacy checkpoint segment count for PostgreSQL releases before 9.5.", value)
	case "io_method":
		return fmt.Sprintf("Set to %s to use worker-based asynchronous I/O.", value)
	case "file_copy_method":
		return fmt.Sprintf("Set to %s as the copy method for file operations.", value)
	default:
		return fmt.Sprintf("Set to %s by the tuning policy for the %s workload profile.", value, request.Profile)
	}
}

func sharedBuffersReason(value string, request TuningRequest, adjustments []ruleAdjustment) string {
	reasons := make([]string, 0, len(adjustments))
	for _, adjustment := range adjustments {
		switch adjustment.rule {
		case ruleArchitecture:
			reasons = append(reasons, capAdjustmentReason(adjustment, "for the 32-bit architecture"))
		case ruleOperatingSystem:
			reasons = append(reasons, capAdjustmentReason(adjustment, "for PostgreSQL 9.6 or earlier"))
		case ruleProfile:
			reasons = append(reasons, fmt.Sprintf("Adjusted from %s to %s by the DESKTOP profile", adjustment.before, adjustment.after))
		case rulePostgreSQLVersion:
			reasons = append(reasons, capAdjustmentReason(adjustment, "for older PostgreSQL performance"))
		}
	}
	if len(reasons) > 0 {
		return fmt.Sprintf("Set to %s. %s.", value, strings.Join(reasons, " Then "))
	}
	return fmt.Sprintf("Set to %s from the memory share for the %s profile.", value, request.Profile)
}

func memoryCapReason(value string, adjustments []ruleAdjustment) (string, bool) {
	reasons := make([]string, 0, len(adjustments))
	for _, adjustment := range adjustments {
		switch adjustment.rule {
		case ruleArchitecture:
			reasons = append(reasons, capAdjustmentReason(adjustment, "for the 32-bit architecture"))
		case ruleOperatingSystem:
			reasons = append(reasons, capAdjustmentReason(adjustment, "by the PostgreSQL pre-18 Windows limit"))
		}
	}
	if len(reasons) > 0 {
		return fmt.Sprintf("Set to %s. %s.", value, strings.Join(reasons, " Then ")), true
	}
	return "", false
}

func capAdjustmentReason(adjustment ruleAdjustment, context string) string {
	if adjustment.before == adjustment.after {
		return fmt.Sprintf("Capped at %s %s", adjustment.after, context)
	}
	return fmt.Sprintf("Capped from %s to %s %s", adjustment.before, adjustment.after, context)
}

func ioWorkersReason(value string, request TuningRequest) string {
	calculation := calculateIOWorkers(request.TotalCPU, request.Profile, request.DiskType)
	reason := fmt.Sprintf("Set to %s from %d logical CPUs and the %s profile", value, request.TotalCPU, request.Profile)
	if calculation.HDDAdjusted {
		reason += " with the HDD storage adjustment"
	}
	reason += "."

	if calculation.MinimumApplied {
		reason += fmt.Sprintf(" The initial %d-worker calculation was raised to the minimum of 2.", calculation.InitialValue)
	}
	if calculation.LogicalCPUCapApplied {
		reason += fmt.Sprintf(" It was then capped at %d to avoid exceeding the logical CPU count.", request.TotalCPU)
	}
	return reason
}
