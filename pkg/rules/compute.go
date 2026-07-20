package rules

import (
	"fmt"
	"reflect"
	"strings"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
)

type rule func(*input.Input, *category.ExportCfg) (*category.ExportCfg, error)

type ruleID string

const (
	ruleArchitecture      ruleID = "architecture"
	ruleOperatingSystem   ruleID = "operating system"
	ruleProfile           ruleID = "profile"
	ruleStorage           ruleID = "storage"
	ruleAsynchronousIO    ruleID = "asynchronous I/O"
	rulePostgreSQLVersion ruleID = "PostgreSQL Version"
)

type namedRule struct {
	id    ruleID
	apply rule
}

type ruleAdjustment struct {
	rule   ruleID
	before string
	after  string
}

type parameterSnapshot struct {
	raw       any
	formatted string
}

var allRules = []namedRule{
	{id: ruleArchitecture, apply: computeArch},
	{id: ruleOperatingSystem, apply: computeOS},
	{id: ruleProfile, apply: computeProfile},
	{id: ruleStorage, apply: computeStorage},
	{id: ruleAsynchronousIO, apply: computeAIO},

	// computeVersion can remove values deppending on the version
	// to be sure that it will not break other rules, leave it at
	// the end.
	{id: rulePostgreSQLVersion, apply: computeVersion},
}

// Compute evaluate all parameters
func Compute(in input.Input) (*category.ExportCfg, error) {
	out, _, err := computeWithAdjustments(in)
	return out, err
}

// computeWithAdjustments is the single calculation pipeline shared by legacy
// consumers and the provenance-aware tuning operation.
func computeWithAdjustments(in input.Input) (*category.ExportCfg, map[string][]ruleAdjustment, error) {
	var (
		out *category.ExportCfg
		err error
	)

	out = category.NewExportCfg(in)
	adjustments := make(map[string][]ruleAdjustment)

	for _, currentRule := range allRules {
		before := parameterValues(out, in.PostgresVersion)

		out, err = currentRule.apply(&in, out)

		if err != nil {
			return nil, nil, fmt.Errorf("could not process rule: %w", err)
		}
		after := parameterValues(out, in.PostgresVersion)
		for name, afterValue := range after {
			beforeValue, existed := before[name]
			if existed && !reflect.DeepEqual(beforeValue.raw, afterValue.raw) {
				adjustments[name] = append(adjustments[name], ruleAdjustment{
					rule: currentRule.id, before: beforeValue.formatted, after: afterValue.formatted,
				})
			}
		}
	}

	return out, adjustments, nil
}

func parameterValues(cfg *category.ExportCfg, postgresVersion float32) map[string]parameterSnapshot {
	formattedValues := make(map[string]string)
	for _, group := range cfg.ToSlice(postgresVersion, false, "") {
		for _, parameter := range group.Parameters {
			formattedValues[parameter.Name] = parameter.Value
		}
	}

	values := make(map[string]parameterSnapshot)
	categories := reflect.ValueOf(cfg).Elem()
	for categoryIndex := 0; categoryIndex < categories.NumField(); categoryIndex++ {
		categoryValue := categories.Field(categoryIndex)
		if categoryValue.IsNil() {
			continue
		}
		parameters := categoryValue.Elem()
		parameterType := parameters.Type()
		for parameterIndex := 0; parameterIndex < parameters.NumField(); parameterIndex++ {
			name := strings.Split(parameterType.Field(parameterIndex).Tag.Get("json"), ",")[0]
			values[name] = parameterSnapshot{
				raw:       parameters.Field(parameterIndex).Interface(),
				formatted: formattedValues[name],
			}
		}
	}
	return values
}
