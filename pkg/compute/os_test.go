package compute

import (
	"testing"
)

// computeOS(computeCPU(BLA(input,param)))

func fakeInput() *Input {
	return NewInput("linux", "x86_64", 4*GB, "WEB", "SSD", 100, 12.2)
}

func Test_computeOS(t *testing.T) {

	_, err := computeOS(Input{OS: "xpto"}, ExportCfg{})

	if err == nil {
		t.Error("should support only windows, linux and unix")
	}

	in := fakeInput()
	in.OS = "windows"
	in.PostgresVersion = 9.6

	out, _ := computeOS(*in, *NewExportCfg(*in))

	if out.Memory.SharedBuffers > 512*MB {
		t.Error("should limit shared_buffers to 512MB until pg 10 on windows")
	}

	in = fakeInput()
	in.TotalRAM = 120 * GB

	out, _ = computeOS(*in, *NewExportCfg(*in))

	if out.Memory.SharedBuffers < 25*GB {
		t.Error("should not limit shared_buffers on versions greater or equal than pg 11")
	}
}
