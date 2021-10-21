package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func Test_computeOS(t *testing.T) {
	_, err := computeOS(&config.Input{OS: "xpto-wrong-os"}, &category.ExportCfg{})

	if err == nil {
		t.Error("should support only windows, linux and unix")
	}

	in := fakeInput()
	in.OS = "windows"
	in.PostgresVersion = 9.6

	out, _ := computeOS(in, category.NewExportCfg(*in))

	if out.Memory.SharedBuffers > 512*config.MB {
		t.Error("should limit shared_buffers to 512MB until pg 10 on windows")
	}

	in = fakeInput()
	in.OS = "windows"
	in.PostgresVersion = 12.0

	out, _ = computeOS(in, category.NewExportCfg(*in))

	if out.Storage.EffectiveIOConcurrency > 0 {
		t.Error("should limit effective_io_concurrency to 0 on platforms that lack posix_fadvise()")
	}

	in = fakeInput()
	in.TotalRAM = 120 * config.GB

	out, _ = computeOS(in, category.NewExportCfg(*in))

	if out.Memory.SharedBuffers < 25*config.GB {
		t.Error("should not limit shared_buffers on versions greater or equal than pg 11")
	}
}
