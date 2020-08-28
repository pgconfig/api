package compute

import (
	"fmt"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

// computeVersion will remove the values that were removed on specific versions
func computeVersion(in *config.Input, cfg *category.ExportCfg, err error) (*config.Input, *category.ExportCfg, error) {

	if err != nil {
		return nil, nil, fmt.Errorf("could not compute Version: %w", err)
	}

	if in.PostgresVersion < 9.5 {
		cfg.Checkpoint.MinWALSize = 0
		cfg.Checkpoint.MaxWALSize = 0
	}

	if in.PostgresVersion < 9.6 {
		cfg.Worker.MaxParallelWorkerPerGather = 0
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

	return in, cfg, nil
}
