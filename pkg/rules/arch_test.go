package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func Test_computeArch(t *testing.T) {

	_, err := computeArch(&config.Input{Arch: "xpto-invalid-arch"}, nil)

	if err == nil {
		t.Error("should support only 386, amd64, arm and arm64")
	}

	in := fakeInput()
	in.Arch = "386"
	in.TotalRAM = 1 * config.TB

	out, _ := computeArch(in, category.NewExportCfg(*in))

	if out.Memory.SharedBuffers > 4*config.GB ||
		out.Memory.WorkMem > 4*config.GB ||
		out.Memory.MaintenanceWorkMem > 4*config.GB {
		t.Error("should limit ANY memory parameter to a max of 4GB in a 32 bits system")
	}

}
