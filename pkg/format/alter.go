package format

import (
	"bytes"
	"fmt"

	"github.com/pgconfig/api/pkg/category"
)

// AlterSystem generates a string with the sql file contents of
// the tuning report in the Slice format
func AlterSystem(report []category.SliceOutput) string {
	var b bytes.Buffer

	for _, cat := range report {
		b.WriteString(fmt.Sprintf("-- %s\n", cat.Description))
		for _, param := range cat.Parameters {
			b.WriteString(fmt.Sprintf("ALTER SYSTEM SET %s TO '%s';\n", param.Name, param.Value))
		}
		b.WriteString("\n")
	}

	return b.String()
}
