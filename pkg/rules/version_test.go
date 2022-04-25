package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input/bytes"
)

func Test_computeVersion(t *testing.T) {
	in := fakeInput()
	in.PostgresVersion = 9.4

	out, _ := computeVersion(in, category.NewExportCfg(*in))

	if out.Checkpoint.MinWALSize > 0 || out.Checkpoint.MaxWALSize > 0 {
		t.Error("should remove min_wal_size and  max_wal_size on versions older than 9.5")
	}

	in = fakeInput()
	in.PostgresVersion = 9.5

	out, _ = computeVersion(in, category.NewExportCfg(*in))

	if out.Checkpoint.CheckpointSegments > 0 {
		t.Error("should remove checkpoint_segments on versions greater than 9.5")
	}

	in = fakeInput()
	in.PostgresVersion = 9.3

	out, _ = computeVersion(in, category.NewExportCfg(*in))

	if out.Worker != nil {
		t.Error("should remove the workers category in versions older than 9.3")
	}

	in = fakeInput()
	in.PostgresVersion = 9.4

	out, _ = computeVersion(in, category.NewExportCfg(*in))

	if out.Worker.MaxParallelWorkerPerGather != 0 {
		t.Error("should remove max_parallel_workers_per_gather on versions < 9.6")
	}

	in = fakeInput()
	in.PostgresVersion = 9.5

	out, _ = computeVersion(in, category.NewExportCfg(*in))

	if out.Worker.MaxParallelWorkers != 0 {
		t.Error("should remove max_parallel_workers on versions < 10")
	}

	in = fakeInput()
	in.PostgresVersion = 9.5
	in.TotalRAM = 1 * bytes.TB

	out, _ = computeVersion(in, category.NewExportCfg(*in))

	if out.Memory.SharedBuffers > 8*bytes.GB {
		t.Error("should limit shared_buffers up to 8gb on versions <= 9.5")
	}
}
