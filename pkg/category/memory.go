package category

import "github.com/pgconfig/api/pkg/config"

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      int
	EffectiveCacheSize int
	WorkMem            int
	MaintenanceWorkMem int
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
