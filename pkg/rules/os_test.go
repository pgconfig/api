package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"

	. "github.com/smartystreets/goconvey/convey"
)

func Test_computeOS(t *testing.T) {

	Convey("Operating systems", t, func() {
		Convey("should return error on non-supported operating systems", func() {
			_, err := computeOS(&input.Input{OS: "xpto-wrong-os"}, &category.ExportCfg{})
			So(err, ShouldResemble, errors.ErrorInvalidOS)
		})

		Convey("should ignore case for all operating systems supported", func() {
			in := fakeInput()
			in.OS = "lINUx"
			in.TotalRAM = 120 * bytes.GB

			_, err := computeOS(in, category.NewExportCfg(*in))
			So(err, ShouldBeNil)
		})

		Convey("should limit shared_buffers to 512MB until pg 10 on windows", func() {
			in := fakeInput()
			in.OS = "windows"
			in.PostgresVersion = 9.6

			out, err := computeOS(in, category.NewExportCfg(*in))
			So(err, ShouldBeNil)
			So(out.Memory.SharedBuffers, ShouldEqual, 512*bytes.MB)
		})

		Convey("should limit effective_io_concurrency to 0 on platforms that lack posix_fadvise()", func() {
			in := fakeInput()
			in.OS = Windows
			in.PostgresVersion = 12.0

			out, err := computeOS(in, category.NewExportCfg(*in))
			So(err, ShouldBeNil)
			So(out.Storage.EffectiveIOConcurrency, ShouldEqual, 0)
		})

		Convey("should not limit shared_buffers on versions greater or equal than pg 11", func() {
			in := fakeInput()
			in.PostgresVersion = 14.0
			in.TotalRAM = 120 * bytes.GB

			out, err := computeOS(in, category.NewExportCfg(*in))
			So(err, ShouldBeNil)
			So(out.Memory.SharedBuffers, ShouldBeGreaterThan, 25*bytes.GB)
		})
	})
}
