package format

import (
	"encoding/json"

	"github.com/pgconfig/api/pkg/category"
)

// JSONFile generates a string with the JSON file contents of
// the tuning report in the Slice format
func JSONFile(report []category.SliceOutput) string {
	b, _ := json.MarshalIndent(report, "", "  ")

	return string(b)
}
