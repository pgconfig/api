package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
)

// computeVersion will remove the values that were removed on specific versions
func computeVersion(in *input.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	if in.PostgresVersion < 9.5 {
		cfg.Checkpoint.MinWALSize = 0
		cfg.Checkpoint.MaxWALSize = 0
	}

	if in.PostgresVersion < 9.6 {
		cfg.Worker.MaxParallelWorkerPerGather = 0

		/*
			until 9.6 (ref to this commit: https://github.com/postgres/postgres/commit/48354581a49c30f5757c203415aa8412d85b0f70)
			large values in this parameter tend to cause slowness
		*/
		if cfg.Memory.SharedBuffers > 8*bytes.GB {
			cfg.Memory.SharedBuffers = 8 * bytes.GB
		}
	}

	if in.PostgresVersion < 10.0 {
		cfg.Worker.MaxParallelWorkers = 0
	}

	if in.PostgresVersion >= 9.5 {
		cfg.Checkpoint.CheckpointSegments = 0
	}

	if in.PostgresVersion <= 9.3 {
		cfg.Worker = nil
	}

	return cfg, nil
}
