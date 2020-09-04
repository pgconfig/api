package compute

import (
	"fmt"

	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func ValidArch(arch string) error {
	switch arch {
	case "x86_64":
	case "i686":
	default:
		return errors.ErrorInvalidArch
	}
	return nil
}

func computeArch(in *config.Input, cfg *category.ExportCfg, err error) (*config.Input, *category.ExportCfg, error) {

	if err != nil {
		return nil, nil, fmt.Errorf("could not compute Arch: %w", err)
	}

	if err = ValidArch(in.Arch); err != nil {
		return nil, nil, err
	}

	if in.Arch == "i686" {
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

	return in, cfg, nil
}
