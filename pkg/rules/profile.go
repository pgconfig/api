package rules

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

func computeProfile(in *input.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {
	switch in.Profile {
	case profile.Desktop:
		cfg.Memory.SharedBuffers = bytes.Byte(in.TotalRAM) / 16
	}

	return cfg, nil
}
