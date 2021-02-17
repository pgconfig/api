package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func computeStorage(in *config.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	switch in.DiskType {
	case "SSD":
		cfg.Storage.EffectiveIOConcurrency = 200
	case "SAN":
		cfg.Storage.EffectiveIOConcurrency = 300
	default:
		cfg.Storage.EffectiveIOConcurrency = 2
	}

	if in.DiskType != "HDD" {
		cfg.Storage.RandomPageCost = 1.1
	}

	return cfg, nil
}
