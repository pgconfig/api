package bytes

import (
	"testing"

	. "github.com/pgconfig/api/pkg/tests"
)

func Test_Bytes(t *testing.T) {
	Describe("Byte parsing and formatting", t, func() {
		It("should parse bytes to the postgres byte format", func() {
			input := 10 * GB
			got, err := marshalBytes(&input)
			Expect(err, ShouldBeNil)
			Expect(got, ShouldResemble, []byte(`"10GB"`))
		})

		Context("when formatting bytes to string", func() {
			It("should format all byte units correctly", func() {
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
					got := formatBytes(tt.args)
					Expect(got, ShouldEqual, tt.want)
				}
			})
		})

		Context("when parsing bytes from string", func() {
			It("should parse Bytes", func() {
				got, err := Parse("5B")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 5)

				got, err = Parse("5")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 5)
			})

			It("should parse KiloBytes", func() {
				got, err := Parse("455KB")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 455*KB)
			})

			It("should parse MegaBytes", func() {
				got, err := Parse("1023MB")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 1023*MB)
			})

			It("should parse GigaBytes", func() {
				got, err := Parse("565GB")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 565*GB)
			})

			It("should parse TeraBytes", func() {
				got, err := Parse("396TB")
				Expect(err, ShouldBeNil)
				Expect(got, ShouldEqual, 396*TB)
			})
		})
	})
}
