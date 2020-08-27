package compute

import "errors"

func computeArch(in *Input, cfg *ExportCfg, err error) (*Input, *ExportCfg, error) {

	switch in.Arch {
	case "x86_64":
	case "i686":
	default:
		return nil, nil, errors.New("Invalid Architecture")
	}

	if in.Arch == "i686" {
		if cfg.Memory.SharedBuffers > 4*GB {
			cfg.Memory.SharedBuffers = 4 * GB
		}
		if cfg.Memory.WorkMem > 4*GB {
			cfg.Memory.WorkMem = 4 * GB
		}
		if cfg.Memory.MaintenanceWorkMem > 4*GB {
			cfg.Memory.MaintenanceWorkMem = 4 * GB
		}
	}

	return in, cfg, nil
}
