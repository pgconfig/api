package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"

	. "github.com/smartystreets/goconvey/convey"
)

func Test_computeArch(t *testing.T) {

	Convey("Validations", t, func() {
		Convey("Should thow an error when the arch is invalid", func() {
			_, err := computeArch(&config.Input{Arch: "xpto-invalid-arch"}, nil)
			So(err, ShouldNotBeNil)
		})
		Convey("Should thow an error when the arch is 386 or i686 and has memory values over 4GiB", func() {

			similarArchs := []string{"386", "i686"}

			for _, newArch := range similarArchs {
				in := fakeInput()
				in.Arch = newArch
				in.TotalRAM = 1 * config.TB

				out, _ := computeArch(in, category.NewExportCfg(*in))
				So(out.Memory.SharedBuffers, ShouldBeLessThanOrEqualTo, 4*config.GB)
				So(out.Memory.WorkMem, ShouldBeLessThanOrEqualTo, 4*config.GB)
				So(out.Memory.MaintenanceWorkMem, ShouldBeLessThanOrEqualTo, 4*config.GB)
			}

		})
	})
}
