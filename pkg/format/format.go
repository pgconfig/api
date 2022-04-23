package format

import (
	"fmt"
	"strings"
)

type ExportFormat string

const (
	JSON              ExportFormat = "JSON"
	Config            ExportFormat = "CONF"
	UNIX              ExportFormat = "UNIX"
	AlterSystemFormat ExportFormat = "ALTER-SYSTEM"
	SQL               ExportFormat = "SQL"
	StackGres         ExportFormat = "STACKGRES"
	StackGresShort    ExportFormat = "SG"
	SGPGConfig        ExportFormat = "SGPOSTGRESCONFIG"
	YAML              ExportFormat = "YAML"
)

// AllExportFormats Lists all of the export options available
var AllExportFormats = []ExportFormat{JSON, Config, UNIX, AlterSystemFormat, SQL, StackGres, StackGresShort, SGPGConfig, YAML}

// String is used both by fmt.Print and by Cobra in help text
func (e *ExportFormat) String() string {
	return string(*e)
}

// Set must have pointer receiver so it doesn't change the value of a copy
func (e *ExportFormat) Set(v string) error {

	newV := ExportFormat(strings.ToUpper(v))

	switch newV {
	case JSON, Config, UNIX, AlterSystemFormat, SQL, StackGres, StackGresShort, SGPGConfig, YAML:
		*e = newV
		return nil
	default:
		return fmt.Errorf("must be one of %v", AllExportFormats)
	}
}

// Type is only used in help text
func (e *ExportFormat) Type() string {
	return "ExportFormat"
}
