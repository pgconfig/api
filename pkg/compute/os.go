package compute

import "errors"

func computeOS(in *Input, cfg *ExportCfg, err error) (*Input, *ExportCfg, error) {

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
