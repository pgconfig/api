package bytes

import (
	"fmt"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func Test_Bytes(t *testing.T) {
	Convey("Parsing", t, func() {
		Convey("should parse bytes to the postgres byte format", func() {
			input := 10 * GB
			got, err := marshalBytes(&input)
			So(err, ShouldBeNil)
			So(got, ShouldResemble, []byte(`"10GB"`))
		})

		Convey("should format bytes to string", func() {
			tests := []struct {
				desc string
				args Byte
				want string
			}{
				{"negative values", -1, "-1"},
				{"zero", 0, "0"},
				{"Bytes", 5, "5B"},
				{"KiloBytes", 455 * KB, "455kB"},
				{"MegaBytes", 1023 * MB, "1023MB"},
				{"GigaBytes", 565 * GB, "565GB"},
				{"TeraBytes", 396 * TB, "396TB"},
			}
			for _, tt := range tests {
				Convey(fmt.Sprintf("should format %s", tt.desc), func() {
					got := formatBytes(tt.args)
					So(got, ShouldEqual, tt.want)
				})
			}
		})

		Convey("should parse bytes from string", func() {
			Convey("should parse Bytes", func() {

				got, err := Parse("5B")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 5)

				got, err = Parse("5")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 5)
			})
			Convey("should parse KiloBytes", func() {

				got, err := Parse("455KB")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 455*KB)
			})
			Convey("should parse MegaBytes", func() {

				got, err := Parse("1023MB")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 1023*MB)
			})
			Convey("should parse GigaBytes", func() {

				got, err := Parse("565GB")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 565*GB)
			})
			Convey("should parse TeraBytes", func() {

				got, err := Parse("396TB")
				So(err, ShouldBeNil)
				So(got, ShouldEqual, 396*TB)
			})
		})
	})
}
