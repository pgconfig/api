package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeArch(t *testing.T) {
	Describe("Architecture validations", t, func() {
		Context("when arch is invalid", func() {
			It("should return an error", func() {
				_, err := computeArch(&input.Input{Arch: "xpto-invalid-arch"}, nil)
				Expect(err, ShouldNotBeNil)
			})
		})

		Context("when arch is 386 or i686 with large memory", func() {
			It("should limit memory values to 4GiB", func() {
				similarArchs := []string{"386", "i686"}

				for _, newArch := range similarArchs {
					in := fakeInput()
					in.Arch = newArch
					in.TotalRAM = 1 * bytes.TB

					out, _ := computeArch(in, category.NewExportCfg(*in))
					Expect(out.Memory.SharedBuffers, ShouldBeLessThanOrEqualTo, 4*bytes.GB)
					Expect(out.Memory.WorkMem, ShouldBeLessThanOrEqualTo, 4*bytes.GB)
					Expect(out.Memory.MaintenanceWorkMem, ShouldBeLessThanOrEqualTo, 4*bytes.GB)
				}
			})
		})
	})
}
