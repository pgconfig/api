package compute

import (
	"fmt"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/errors"
)

func ValidOS(os string) error {
	switch os {
	case "windows":
	case "linux":
	case "unix":
	default:
		return errors.ErrorInvalidOS
	}
	return nil
}

func computeOS(in *config.Input, cfg *category.ExportCfg, err error) (*config.Input, *category.ExportCfg, error) {

	if err != nil {
		return nil, nil, fmt.Errorf("could not compute OS: %w", err)
	}

	if err = ValidOS(in.OS); err != nil {
		return nil, nil, err
	}

	if in.PostgresVersion <= 9.6 {
		cfg.Memory.SharedBuffers = 512 * config.MB
	}

	return in, cfg, nil
}
