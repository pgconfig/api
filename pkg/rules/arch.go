package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/errors"
)

// ValidArch validates the arch
func ValidArch(arch string) error {
	switch arch {
	case "386":
	case "i686":
	case "amd64":
	case "x86-64":
	case "arm":
	case "arm64":
	default:
		return errors.ErrorInvalidArch
	}
	return nil
}

func computeArch(in *config.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	if err := ValidArch(in.Arch); err != nil {
		return nil, err
	}

	if in.Arch == "386" || in.Arch == "i686" {
		if cfg.Memory.SharedBuffers > 4*config.GB {
			cfg.Memory.SharedBuffers = 4 * config.GB
		}
		if cfg.Memory.WorkMem > 4*config.GB {
			cfg.Memory.WorkMem = 4 * config.GB
		}
		if cfg.Memory.MaintenanceWorkMem > 4*config.GB {
			cfg.Memory.MaintenanceWorkMem = 4 * config.GB
		}
	}

	return cfg, nil
}
