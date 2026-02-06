package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeProfile(t *testing.T) {
	Describe("Profile computation", t, func() {
		It("should apply lower shared_buffers for Desktop profile", func() {
			in := fakeInput()
			in.Profile = profile.Desktop
			in.TotalRAM = 4 * bytes.GB

			out, _ := computeProfile(in, category.NewExportCfg(*in))

			expected := (4 * bytes.GB) / 16
			Expect(out.Memory.SharedBuffers, ShouldEqual, expected)
		})
	})
}
