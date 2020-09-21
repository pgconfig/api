package category

import "github.com/pgconfig/api/pkg/config"

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      config.Byte `json:"shared_buffers"`
	EffectiveCacheSize config.Byte `json:"effective_cache_size"`
	WorkMem            config.Byte `json:"work_mem"`
	MaintenanceWorkMem config.Byte `json:"maintenance_work_mem"`
}

// NewMemoryCfg creates a new Memory Configuration
func NewMemoryCfg(in config.Input) *MemoryCfg {
	return &MemoryCfg{
		SharedBuffers:      config.Byte(in.TotalRAM) / 4,
		EffectiveCacheSize: (config.Byte(in.TotalRAM) / 4) * 3,
		WorkMem:            in.TotalRAM / config.Byte(in.MaxConnections),
		MaintenanceWorkMem: config.Byte(in.TotalRAM) / 16,
	}
}
