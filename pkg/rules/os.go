package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
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

func computeOS(in *input.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	var err error

	if err = ValidOS(in.OS); err != nil {
		return nil, err
	}

	if cfg.Memory.SharedBuffers > 512*bytes.MB && in.PostgresVersion <= 9.6 {
		cfg.Memory.SharedBuffers = 512 * bytes.MB
	}

	if in.OS == "windows" {
		cfg.Storage.EffectiveIOConcurrency = 0
	}

	return cfg, nil
}
