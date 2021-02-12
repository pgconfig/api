package category

import (
	"github.com/pgconfig/api/pkg/config"
)

// ExportCfg is the final report
type ExportCfg struct {
	Memory     *MemoryCfg     `id:"memory_related" desc:"Memory Configuration"`
	Checkpoint *CheckpointCfg `id:"checkpoint_related" desc:"Checkpoint Related Configuration"`
	Network    *NetworkCfg    `id:"network_related" desc:"Network Related Configuration"`
	Storage    *StorageCfg    `id:"storage_type" desc:"Storage Configuration"`
	Worker     *WorkerCfg     `id:"worker_related" desc:"Worker Processes Configuration,omitempty"`
}

// NewExportCfg creates a new ExportCfg with the basic values
// to be processed by the input rules
func NewExportCfg(in config.Input) *ExportCfg {
	return &ExportCfg{
		Memory:     NewMemoryCfg(in),
		Checkpoint: NewCheckpointCfg(in),
		Network:    NewNetworkCfg(in),
		Storage:    NewStorageCfg(in),
		Worker:     NewWorkerCfg(in),
	}
}
