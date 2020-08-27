package category

import "github.com/pgconfig/api/pkg/config"

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      int `json:"shared_buffers"`
	EffectiveCacheSize int `json:"effective_cache_size"`
	WorkMem            int `json:"work_mem"`
	MaintenanceWorkMem int `json:"maintenance_work_mem"`
}

// NewMemoryCfg creates a new Memory Configuration
func NewMemoryCfg(in config.Input) *MemoryCfg {
	return &MemoryCfg{
		SharedBuffers:      in.TotalRAM / 4,
		EffectiveCacheSize: (in.TotalRAM / 4) * 3,
		WorkMem:            (in.TotalRAM / in.MaxConnections),
		MaintenanceWorkMem: in.TotalRAM / 16,
	}
}
