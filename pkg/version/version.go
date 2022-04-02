package version

import "fmt"

var (
	// Tag is the current $(git tag) that this code is built
	Tag = "development"

	// Commit is the current commit that this code is built
	Commit = "latest"
)

// Pretty formats the version with commit and tag
func Pretty() string {
	return fmt.Sprintf("%s (%s)", Tag, Commit)
}
