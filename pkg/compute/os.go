package compute

import (
	"errors"
	"fmt"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

func computeOS(in *config.Input, cfg *category.ExportCfg, err error) (*config.Input, *category.ExportCfg, error) {

	if err != nil {
		return nil, nil, fmt.Errorf("could not compute OS: %w", err)
	}

	switch in.OS {
	case "windows":
	case "linux":
	case "unix":
	default:
		return nil, nil, errors.New("Invalid OS")
	}

	if in.PostgresVersion <= 9.6 {
		cfg.Memory.SharedBuffers = 512 * config.MB
	}

	return in, cfg, nil
}
