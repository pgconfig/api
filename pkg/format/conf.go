package format

import (
	"bytes"
	"fmt"

	"github.com/pgconfig/api/pkg/category"
)

// ConfigFile generates a string with the config file contents of
// the tuning report in the Slice format
func ConfigFile(report []category.SliceOutput) string {
	var b bytes.Buffer

	for _, cat := range report {
		b.WriteString(fmt.Sprintf("# %s\n", cat.Description))
		for _, param := range cat.Parameters {

			if param.Comment != "" {
				b.WriteString(fmt.Sprintf("\n# %s\n", param.Comment))
			}

			if param.Format == "string" {
				b.WriteString(fmt.Sprintf("%s = '%s'\n", param.Name, param.Value))
				continue
			}

			b.WriteString(fmt.Sprintf("%s = %s\n", param.Name, param.Value))
		}
		b.WriteString("\n")
	}

	return b.String()
}
