package category

import (
	"github.com/pgconfig/api/pkg/config"
)

// ExportCfg is the final report
type ExportCfg struct {
	Memory *MemoryCfg
}

// NewExportCfg creates a new ExportCfg with the basic values
// to be processed by the input rules
func NewExportCfg(in config.Input) *ExportCfg {
	return &ExportCfg{
		Memory: NewMemoryCfg(in),
	}
}
