package compute

import (
	"errors"
	"testing"
)

// computeOS(computeCPU(BLA(input,param)))

func fakeInput() *Input {
	return NewInput("linux", "x86_64", 4*GB, "WEB", "SSD", 100, 12.2)
}

func shouldAbortChainOnError(origin func(*Input, *ExportCfg, error) (*Input, *ExportCfg, error), t *testing.T) {

	_, _, errOut := origin(nil, nil, errors.New("error"))

	if errOut == nil {
		t.Error("should abort the compute chain when a error happens")
	}
}

func Test_computeOS(t *testing.T) {

	shouldAbortChainOnError(computeOS, t)

	_, _, err := computeOS(&Input{OS: "xpto-wrong-os"}, &ExportCfg{}, nil)

	if err == nil {
		t.Error("should support only windows, linux and unix")
	}

	in := fakeInput()
	in.OS = "windows"
	in.PostgresVersion = 9.6

	_, out, _ := computeOS(in, NewExportCfg(*in), nil)

	if out.Memory.SharedBuffers > 512*MB {
		t.Error("should limit shared_buffers to 512MB until pg 10 on windows")
	}

	in = fakeInput()
	in.TotalRAM = 120 * GB

	_, out, _ = computeOS(in, NewExportCfg(*in), nil)

	if out.Memory.SharedBuffers < 25*GB {
		t.Error("should not limit shared_buffers on versions greater or equal than pg 11")
	}
}
