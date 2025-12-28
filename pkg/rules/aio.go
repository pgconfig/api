package rules

import (
	"math"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/profile"
)

func computeAIO(in *input.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {
	// Set default values already from NewStorageCfg
	// Adjust io_workers based on profile and CPU cores

	// Factor per profile
	var factor float64
	switch in.Profile {
	case profile.Desktop:
		factor = 0.1
	case profile.Web:
		factor = 0.2
	case profile.Mixed:
		factor = 0.25
	case profile.OLTP:
		factor = 0.3
	case profile.DW:
		factor = 0.4
	default:
		factor = 0.25
	}

	// Adjust factor based on disk type (optional)
	// HDD may benefit from more workers, SSD from fewer
	switch in.DiskType {
	case "HDD":
		factor += 0.1
	case "SSD", "SAN":
		// keep factor as is
	}

	// Calculate io_workers
	workers := int(math.Ceil(float64(in.TotalCPU) * factor))
	// Ensure at least 2 workers, but keep default 3 as minimum
	if workers < 2 {
		workers = 2
	}
	// Max workers? Not needed, but can limit to total CPU
	if workers > in.TotalCPU {
		workers = in.TotalCPU
	}
	cfg.Storage.IOWorkers = workers

	// Tune io_max_combine_limit and io_max_concurrency based on profile
	// Values assume 8KB pages for limits (16 = 128KB, 128 = 1MB)
	switch in.Profile {
	case profile.DW:
		// Data Warehouse benefits from larger I/O chunks and higher concurrency
		cfg.Storage.IOMaxCombineLimit = 128 // 1MB
		cfg.Storage.IOMaxConcurrency = 256
	case profile.OLTP:
		// OLTP benefits from concurrency but keep chunks standard
		cfg.Storage.IOMaxConcurrency = 128
	}

	// Optionally set io_method based on OS and profile
	// For now keep default "worker"
	// If Linux and profile is DW or OLTP, consider io_uring
	// but we need to be careful about container environments
	// cfg.Storage.IOMethod = "worker"

	return cfg, nil
}