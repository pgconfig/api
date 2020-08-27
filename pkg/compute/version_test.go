package compute

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
)

func Test_computeVersion(t *testing.T) {
	shouldAbortChainOnError(computeVersion, t)

	in := fakeInput()
	in.PostgresVersion = 9.4

	_, out, _ := computeVersion(in, category.NewExportCfg(*in), nil)

	if out.Checkpoint.MinWALSize > 0 || out.Checkpoint.MaxWALSize > 0 {
		t.Error("should remove min_wal_size and  max_wal_size on versions older than 9.5")
	}

	in = fakeInput()
	in.PostgresVersion = 9.5

	_, out, _ = computeVersion(in, category.NewExportCfg(*in), nil)

	if out.Checkpoint.CheckpointSegments > 0 {
		t.Error("should remove checkpoint_segments on versions greater than 9.5")
	}
}
