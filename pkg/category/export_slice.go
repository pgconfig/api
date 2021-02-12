package category

import (
	"fmt"
	"reflect"
	"strings"

	"github.com/pgconfig/api/pkg/config"
)

// ToSlice converts de report into a slice
// with categories and parameters like that is used today on the
// api.pgconfig website.
func (e *ExportCfg) ToSlice() []SliceOutput {

	var out []SliceOutput

	t := reflect.TypeOf(e).Elem()

	v := reflect.ValueOf(e).Elem()
	// typeOfT := v.Type()

	for i := 0; i < v.NumField(); i++ {
		f := v.Field(i)

		id, _ := t.Field(i).Tag.Lookup("id")
		desc, _ := t.Field(i).Tag.Lookup("desc")

		out = append(out, SliceOutput{
			Name:        strings.Split(id, ",")[0],
			Description: strings.Split(desc, ",")[0],
			Parameters:  loadParams(f),
		})
		// fmt.Printf("%d: %s %s = %v\n", i,
		// 	typeOfT.Field(i).Name, f.Type(), f.Interface())
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
	Name   string `json:"name"`
	Value  string `json:"config_value"`
	Format string `json:"format"`
}

func loadParams(cat reflect.Value) []ParamSliceOutput {
	var out []ParamSliceOutput

	t := reflect.TypeOf(cat.Interface()).Elem()
	v := reflect.ValueOf(cat.Interface()).Elem()

	for i := 0; i < t.NumField(); i++ {
		f := v.Field(i)

		id, _ := t.Field(i).Tag.Lookup("json")

		out = append(out, ParamSliceOutput{
			Name:   strings.Split(id, ",")[0],
			Value:  formatParam(f),
			Format: f.Type().Name(),
		})
	}

	return out
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
