package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/errors"
)

// ValidOS validates the Operating System
func ValidOS(os string) error {
	switch os {
	case "windows":
	case "linux":
	case "unix", "darwin":
	default:
		return errors.ErrorInvalidOS
	}
	return nil
}

func computeOS(in *config.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	var err error

	if err = ValidOS(in.OS); err != nil {
		return nil, err
	}

	if in.PostgresVersion <= 9.6 {
		cfg.Memory.SharedBuffers = 512 * config.MB
	}

	if in.OS == "windows" {
		cfg.Storage.EffectiveIOConcurrency = 0
	}

	return cfg, nil
}
