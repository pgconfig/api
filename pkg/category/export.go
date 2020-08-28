package category

import (
	"github.com/pgconfig/api/pkg/config"
)

// ExportCfg is the final report
type ExportCfg struct {
	Memory     *MemoryCfg     `json:"Memory Configuration"`
	Checkpoint *CheckpointCfg `json:"Checkpoint Related Configuration"`
	Storage    *StorageCfg    `json:"Storage Configuration"`
	Worker     *WorkerCfg     `json:"Worker Processes Configuration,omitempty"`
}

// NewExportCfg creates a new ExportCfg with the basic values
// to be processed by the input rules
func NewExportCfg(in config.Input) *ExportCfg {
	return &ExportCfg{
		Memory:     NewMemoryCfg(in),
		Checkpoint: NewCheckpointCfg(in),
		Storage:    NewStorageCfg(in),
		Worker:     NewWorkerCfg(in),
	}
}
