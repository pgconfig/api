package compute

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/profile"
)

func computeProfile(in *config.Input, cfg *category.ExportCfg, err error) (*config.Input, *category.ExportCfg, error) {
	switch in.Profile {
	case profile.Desktop:
		cfg.Memory.SharedBuffers = config.Byte(in.TotalRAM) / 16
	}

	return in, cfg, nil
}
