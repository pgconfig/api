package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/profile"
)

func Test_computeProfile(t *testing.T) {
	in := fakeInput()
	in.Profile = profile.Desktop
	in.TotalRAM = 4 * config.GB

	out, _ := computeProfile(in, category.NewExportCfg(*in))

	if in.Profile == profile.Desktop && out.Memory.SharedBuffers != (4*config.GB)/16 {
		t.Error("should apply a lower value for shared_buffers on the Desktop profile")
	}
}
