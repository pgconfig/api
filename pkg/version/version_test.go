package version

import (
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestPretty(t *testing.T) {
	Convey("Version", t, func() {
		Convey("should print the version as expected", func() {
			got := Pretty()
			So(got, ShouldResemble, fmt.Sprintf("%s (%s)", Tag, Commit))
		})
	})
}
