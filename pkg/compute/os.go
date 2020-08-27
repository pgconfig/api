package compute

import (
	"errors"
	"fmt"
)

func computeOS(in *Input, cfg *ExportCfg, err error) (*Input, *ExportCfg, error) {

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
		cfg.Memory.SharedBuffers = 512 * MB
	}

	return in, cfg, nil
}
