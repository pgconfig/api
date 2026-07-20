package rules

import (
	"encoding/json"
	"reflect"
	"strings"
	"testing"

	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

func TestTuneReturnsCanonicalRequestAndRecommendations(t *testing.T) {
	request := TuningRequest{
		OS:                " Linux ",
		Arch:              "AMD64",
		TotalRAM:          32 * bytes.GB,
		Profile:           profile.OLTP,
		DiskType:          "ssd",
		MaxConnections:    100,
		TotalCPU:          16,
		PostgreSQLVersion: "18.4",
	}

	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}

	if result.Request.OS != "linux" || result.Request.Arch != "amd64" || result.Request.DiskType != "SSD" {
		t.Fatalf("request was not normalized: %+v", result.Request)
	}
	if result.Request.PostgreSQLVersion != "18.4" {
		t.Fatalf("complete PostgreSQL Version was not preserved: %+v", result.Request)
	}
	if len(result.Recommendations) == 0 {
		t.Fatal("expected recommendations")
	}
	for name, recommendation := range result.Recommendations {
		if recommendation.Value == "" {
			t.Errorf("%s has no formatted value", name)
		}
		if !strings.Contains(recommendation.Reason, recommendation.Value) {
			t.Errorf("%s reason does not describe final value %q: %q", name, recommendation.Value, recommendation.Reason)
		}
	}
}

func TestTuneReportsLaterCaps(t *testing.T) {
	request := validTuningRequest()
	request.OS = "windows"
	request.TotalRAM = 1 * bytes.TB
	request.Profile = profile.OLTP
	request.MaxConnections = 1
	request.TotalCPU = 16
	request.PostgreSQLVersion = "17.10"
	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}

	for _, name := range []string{"work_mem", "maintenance_work_mem"} {
		recommendation := result.Recommendations[name]
		if !strings.Contains(recommendation.Reason, "Capped") || !strings.Contains(recommendation.Reason, recommendation.Value) {
			t.Errorf("%s does not explain its cap: %+v", name, recommendation)
		}
	}
}

func TestTuneDoesNotReportCapsForNaturallyEqualValues(t *testing.T) {
	tests := []struct {
		name      string
		request   TuningRequest
		parameter string
		value     string
	}{
		{
			name: "work_mem naturally equals architecture limit",
			request: func() TuningRequest {
				r := validTuningRequest()
				r.Arch = "386"
				r.MaxConnections = 1
				return r
			}(),
			parameter: "work_mem", value: "4GB",
		},
		{
			name: "shared_buffers naturally equals architecture limit",
			request: func() TuningRequest {
				r := validTuningRequest()
				r.Arch = "386"
				return r
			}(),
			parameter: "shared_buffers", value: "4GB",
		},
		{
			name: "shared_buffers naturally equals old version limit",
			request: func() TuningRequest {
				r := validTuningRequest()
				r.TotalRAM = 2 * bytes.GB
				r.PostgreSQLVersion = "9.6.24"
				return r
			}(),
			parameter: "shared_buffers", value: "512MB",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := Tune(tt.request)
			if err != nil {
				t.Fatal(err)
			}
			recommendation := result.Recommendations[tt.parameter]
			if recommendation.Value != tt.value {
				t.Fatalf("%s = %q, want %q", tt.parameter, recommendation.Value, tt.value)
			}
			if strings.Contains(strings.ToLower(recommendation.Reason), "capped") {
				t.Fatalf("naturally equal value incorrectly reported as capped: %+v", recommendation)
			}
		})
	}
}

func TestTuneReportsCapWhenRawValuesRenderEqually(t *testing.T) {
	request := validTuningRequest()
	request.Arch = "386"
	request.TotalRAM = 16*bytes.GB + bytes.MB

	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}
	recommendation := result.Recommendations["shared_buffers"]
	if recommendation.Value != "4GB" {
		t.Fatalf("shared_buffers = %q, want %q", recommendation.Value, "4GB")
	}
	if !strings.Contains(recommendation.Reason, "Capped") {
		t.Fatalf("raw-value cap was omitted because both values render as 4GB: %+v", recommendation)
	}
}

func TestTuneExplainsStorageOverrideOnWindows(t *testing.T) {
	request := validTuningRequest()
	request.OS = "windows"
	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}

	recommendation := result.Recommendations["effective_io_concurrency"]
	if recommendation.Value != "200" {
		t.Fatalf("effective_io_concurrency = %q, want %q", recommendation.Value, "200")
	}
	if !strings.Contains(recommendation.Reason, "replaces the initial Windows compatibility value") {
		t.Fatalf("reason does not explain the later storage adjustment: %q", recommendation.Reason)
	}
}

func TestTuneExplainsIOWorkerAdjustments(t *testing.T) {
	tests := []struct {
		name       string
		totalCPU   int
		diskType   string
		contains   []string
		notContain string
	}{
		{
			name: "SSD has no storage adjustment", totalCPU: 8, diskType: "SSD",
			notContain: "storage adjustment",
		},
		{
			name: "HDD applies its worker adjustment", totalCPU: 8, diskType: "HDD",
			contains: []string{"HDD storage adjustment"},
		},
		{
			name: "one CPU applies minimum then CPU cap", totalCPU: 1, diskType: "SSD",
			contains: []string{"minimum of 2", "capped at 1"},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			request := validTuningRequest()
			request.DiskType = tt.diskType
			request.TotalCPU = tt.totalCPU
			result, err := Tune(request)
			if err != nil {
				t.Fatal(err)
			}

			reason := result.Recommendations["io_workers"].Reason
			for _, expected := range tt.contains {
				if !strings.Contains(reason, expected) {
					t.Errorf("reason %q does not contain %q", reason, expected)
				}
			}
			if tt.notContain != "" && strings.Contains(reason, tt.notContain) {
				t.Errorf("reason %q unexpectedly contains %q", reason, tt.notContain)
			}
		})
	}
}

func TestTuneExplainsMaintenanceWorkMemIndependentlyOfConnections(t *testing.T) {
	result, err := Tune(validTuningRequest())
	if err != nil {
		t.Fatal(err)
	}

	reason := result.Recommendations["maintenance_work_mem"].Reason
	if strings.Contains(reason, "connections") {
		t.Fatalf("maintenance_work_mem reason incorrectly depends on connections: %q", reason)
	}
	if !strings.Contains(reason, "5%") {
		t.Fatalf("maintenance_work_mem reason does not explain its memory share: %q", reason)
	}
}

func TestTuneExplainsWorkerFloors(t *testing.T) {
	request := validTuningRequest()
	request.Profile = profile.DW
	request.TotalCPU = 2
	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}

	for _, name := range []string{"max_worker_processes", "max_parallel_workers"} {
		recommendation := result.Recommendations[name]
		if recommendation.Value != "8" || !strings.Contains(recommendation.Reason, "minimum of 8") {
			t.Errorf("%s does not explain its floor: %+v", name, recommendation)
		}
	}
	gather := result.Recommendations["max_parallel_workers_per_gather"]
	if gather.Value != "2" || !strings.Contains(gather.Reason, "half of 2 logical CPUs") || !strings.Contains(gather.Reason, "minimum of 2") {
		t.Errorf("max_parallel_workers_per_gather does not explain its calculation: %+v", gather)
	}
}

func TestTuneExplainsWALBufferConditions(t *testing.T) {
	tests := []struct {
		name     string
		totalRAM bytes.Byte
		value    string
		reason   string
	}{
		{name: "large OLTP shared buffers", totalRAM: 40 * bytes.GB, value: "32MB", reason: "derived shared_buffers exceeds 8GB"},
		{name: "small OLTP shared buffers", totalRAM: 16 * bytes.GB, value: "-1", reason: "automatic PostgreSQL tuning"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			request := validTuningRequest()
			request.TotalRAM = tt.totalRAM
			request.Profile = profile.OLTP
			result, err := Tune(request)
			if err != nil {
				t.Fatal(err)
			}

			recommendation := result.Recommendations["wal_buffers"]
			if recommendation.Value != tt.value || !strings.Contains(recommendation.Reason, tt.reason) {
				t.Fatalf("wal_buffers provenance = %+v, want value %q and reason containing %q", recommendation, tt.value, tt.reason)
			}
		})
	}
}

func TestTuneExplainsPolicyConstants(t *testing.T) {
	request := validTuningRequest()
	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}

	expectedReasons := map[string]string{
		"checkpoint_completion_target": "spread checkpoint I/O",
		"io_method":                    "worker-based asynchronous I/O",
		"file_copy_method":             "copy method for file operations",
	}
	for name, expected := range expectedReasons {
		reason := result.Recommendations[name].Reason
		if !strings.Contains(reason, expected) {
			t.Errorf("%s reason %q does not explain policy constant %q", name, reason, expected)
		}
	}

	request.PostgreSQLVersion = "9.4.26"
	legacyResult, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}
	checkpointSegments := legacyResult.Recommendations["checkpoint_segments"]
	if !strings.Contains(checkpointSegments.Reason, "legacy checkpoint segment count") {
		t.Errorf("checkpoint_segments reason does not explain the compatibility policy: %+v", checkpointSegments)
	}
}

func TestTuneOmitsListenAddressesFromRichRecommendations(t *testing.T) {
	in := validLegacyInput()

	result, err := Tune(NewTuningRequest(in))
	if err != nil {
		t.Fatal(err)
	}
	if _, exists := result.Recommendations["listen_addresses"]; exists {
		t.Fatal("rich recommendations must omit listen_addresses")
	}

	legacyParameters := result.CompatibilityProjection().ToSlice(in.PostgresVersion, false, "")
	for _, group := range legacyParameters {
		for _, parameter := range group.Parameters {
			if parameter.Name == "listen_addresses" && parameter.Value == "*" {
				return
			}
		}
	}
	t.Fatal("compatibility projection must retain listen_addresses")
}

func TestTuneNormalizesPostgreSQLVersionWhitespace(t *testing.T) {
	request := NewTuningRequest(validLegacyInput())
	request.PostgreSQLVersion = " 18.4 "

	result, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}
	if result.Request.PostgreSQLVersion != "18.4" {
		t.Fatalf("PostgreSQL Version = %q, want %q", result.Request.PostgreSQLVersion, "18.4")
	}
}

func TestTuneIsDeterministic(t *testing.T) {
	request := NewTuningRequest(validLegacyInput())

	first, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}
	second, err := Tune(request)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(first, second) {
		t.Fatalf("repeated tuning differs:\n%+v\n%+v", first, second)
	}

	firstJSON, _ := json.Marshal(first)
	secondJSON, _ := json.Marshal(second)
	if string(firstJSON) != string(secondJSON) {
		t.Fatalf("serialized results differ:\n%s\n%s", firstJSON, secondJSON)
	}
}

func TestCompatibilityProjectionMatchesCompute(t *testing.T) {
	in := validLegacyInput()
	legacy, err := Compute(in)
	if err != nil {
		t.Fatal(err)
	}
	result, err := Tune(NewTuningRequest(in))
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(legacy, result.CompatibilityProjection()) {
		t.Fatal("compatibility projection changed the legacy calculation")
	}
}

func validTuningRequest() TuningRequest {
	return TuningRequest{
		OS: "linux", Arch: "amd64", TotalRAM: 16 * bytes.GB,
		Profile: profile.Web, DiskType: "SSD", MaxConnections: 100,
		TotalCPU: 8, PostgreSQLVersion: "18.4",
	}
}

func validLegacyInput() input.Input {
	return input.Input{
		OS: "linux", Arch: "amd64", TotalRAM: 16 * bytes.GB,
		Profile: profile.Web, DiskType: "SSD", MaxConnections: 100,
		TotalCPU: 8, PostgresVersion: 18,
	}
}
