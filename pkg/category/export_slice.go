package category

import (
	"fmt"
	"reflect"
	"strconv"
	"strings"

	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/docs"
)


var PGBadgerConfig = SliceOutput{
	Name: "log_config",
	Description: "Logging configuration for pgbadger",
	Parameters: []ParamSliceOutput{
		ParamSliceOutput{Format:"bool", Name: "logging_collector", Value: "on"},
		ParamSliceOutput{Format:"bool", Name: "log_checkpoints", Value: "on"},
		ParamSliceOutput{Format:"bool", Name: "log_connections", Value: "on"},
		ParamSliceOutput{Format:"bool", Name: "log_disconnections", Value: "on"},
		ParamSliceOutput{Format:"bool", Name: "log_lock_waits", Value: "on"},
		ParamSliceOutput{Format:"int", Name: "log_temp_files", Value: "0"},
		ParamSliceOutput{Format:"string", Name: "lc_messages", Value: "C"},
		ParamSliceOutput{Format:"string", Name: "log_min_duration_statement", Value: "10s", Comment:"Adjust the minimum time to collect the data"},
		ParamSliceOutput{Format:"int", Name: "log_autovacuum_min_duration", Value: "0"},
	},
}

// ToSlice converts de report into a slice
// with categories and parameters like that is used today on the
// api.pgconfig website.
func (e *ExportCfg) ToSlice(pgVersion float32, includePGBadger bool) []SliceOutput {

	var out []SliceOutput

	t := reflect.TypeOf(e).Elem()

	v := reflect.ValueOf(e).Elem()
	// typeOfT := v.Type()

	for i := 0; i < v.NumField(); i++ {
		f := v.Field(i)

		id, _ := t.Field(i).Tag.Lookup("id")
		desc, _ := t.Field(i).Tag.Lookup("desc")

		params := loadParams(f, pgVersion)

		if len(params) == 0 {
			continue
		}

		out = append(out, SliceOutput{
			Name:        strings.Split(id, ",")[0],
			Description: strings.Split(desc, ",")[0],
			Parameters:  params,
		})
		// fmt.Printf("%d: %s %s = %v\n", i,
		// 	typeOfT.Field(i).Name, f.Type(), f.Interface())
	}

	if includePGBadger {
		out = append(out, PGBadgerConfig)
	}

	return out
}

// SliceOutput is the slice output of the categories
type SliceOutput struct {
	Name        string             `json:"category"`
	Description string             `json:"description"`
	Parameters  []ParamSliceOutput `json:"parameters"`
}

// ParamSliceOutput is the parameter representation of
// the categories slice output
type ParamSliceOutput struct {
	Name          string         `json:"name"`
	Value         string         `json:"config_value"`
	Format        string         `json:"format"`
	Documentation *docs.ParamDoc `json:"documentation,omitempty"`
	Comment       string         `json:"comment,omitempty"`
}

func loadParams(cat reflect.Value, pgVersion float32) []ParamSliceOutput {
	var out []ParamSliceOutput

	t := reflect.TypeOf(cat.Interface()).Elem()
	v := reflect.ValueOf(cat.Interface())

	if !v.IsValid() {
		return nil
	}

	for i := 0; i < t.NumField(); i++ {

		if !v.Elem().IsValid() {
			continue
		}

		f := v.Elem().Field(i)

		// fmt.Printf("%+v\n\n", f)

		id, _ := t.Field(i).Tag.Lookup("json")

		if val, ok := t.Field(i).Tag.Lookup("min_version"); ok {

			minVersion := parseVersion(val)

			skip := pgVersion >= minVersion

			// log.Println("id: ", id, "minVer", minVersion, "pgVer", pgVersion, "skip", !skip)

			if !skip {
				continue
			}
		}
		if val, ok := t.Field(i).Tag.Lookup("max_version"); ok {

			maxVersion := parseVersion(val)

			skip := maxVersion >= pgVersion

			// log.Println("id: ", id, "maxVersion", maxVersion, "pgVer", pgVersion, "skip", !skip)

			if !skip {
				continue
			}
		}

		out = append(out, ParamSliceOutput{
			Name:   strings.Split(id, ",")[0],
			Value:  formatParam(f),
			Format: f.Type().Name(),
		})
	}

	return out
}

func parseVersion(v string) float32 {

	pgVersion, err := strconv.ParseFloat(v, 32)

	if err != nil {
		panic(err)
	}

	return float32(pgVersion)
}

func formatParam(p reflect.Value) string {

	switch p.Type().Name() {
	case "Byte":
		val := p.Interface().(config.Byte)
		return val.String()
	case "int":
		val := p.Interface().(int)
		return fmt.Sprintf("%d", val)
	case "float32":
		val := p.Interface().(float32)
		return fmt.Sprintf("%.1f", val)
	case "string":
		val := p.Interface().(string)
		return val
	}

	return "NOT PARSED - UNKNOW"
}
