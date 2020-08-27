package compute

import (
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
)

// Compute evaluate all parameters
func Compute(in config.Input) (*config.Input, *category.ExportCfg, error) {
	return computeArch(
		computeOS(
			computeVersion(&in,
				category.NewExportCfg(in), nil)))
}
