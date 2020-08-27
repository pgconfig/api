package compute

import (
	"errors"
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

// computeOS(computeCPU(BLA(input,param)))

func fakeInput() *config.Input {
	return config.NewInput("linux", "x86_64", 4*config.GB, "WEB", "SSD", 100, 12.2)
}

func shouldAbortChainOnError(origin func(*config.Input, *category.ExportCfg, error) (*config.Input, *category.ExportCfg, error), t *testing.T) {

	_, _, errOut := origin(nil, nil, errors.New("error"))

	if errOut == nil {
		t.Error("should abort the compute chain when a error happens")
	}
}

func Test_computeOS(t *testing.T) {

	shouldAbortChainOnError(computeOS, t)

	_, _, err := computeOS(&config.Input{OS: "xpto-wrong-os"}, &category.ExportCfg{}, nil)

	if err == nil {
		t.Error("should support only windows, linux and unix")
	}

	in := fakeInput()
	in.OS = "windows"
	in.PostgresVersion = 9.6

	_, out, _ := computeOS(in, category.NewExportCfg(*in), nil)

	if out.Memory.SharedBuffers > 512*config.MB {
		t.Error("should limit shared_buffers to 512MB until pg 10 on windows")
	}

	in = fakeInput()
	in.TotalRAM = 120 * config.GB

	_, out, _ = computeOS(in, category.NewExportCfg(*in), nil)

	if out.Memory.SharedBuffers < 25*config.GB {
		t.Error("should not limit shared_buffers on versions greater or equal than pg 11")
	}
}
