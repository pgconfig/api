package version

import (
	"fmt"
	"testing"

	. "github.com/pgconfig/api/pkg/tests"
)

func TestPretty(t *testing.T) {
	Describe("Version", t, func() {
		It("should print the version as expected", func() {
			got := Pretty()
			Expect(got, ShouldResemble, fmt.Sprintf("%s (%s)", Tag, Commit))
		})
	})
}
