package compute

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func Test_computeArch(t *testing.T) {

	shouldAbortChainOnError(computeArch, t)

	_, _, err := computeArch(&config.Input{Arch: "xpto-invalid-arch"}, &category.ExportCfg{}, nil)

	if err == nil {
		t.Error("should support only x86_64 and i686")
	}

	in := fakeInput()
	in.Arch = "i686"
	in.TotalRAM = 1 * TB

	_, out, _ := computeArch(in, category.NewExportCfg(*in), nil)

	if out.Memory.SharedBuffers > 4*GB ||
		out.Memory.WorkMem > 4*GB ||
		out.Memory.MaintenanceWorkMem > 4*GB {
		t.Error("should limit ANY memory parameter to a max of 4GB in a 32 bits system")
	}

}
