package category

import (
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/profile"
)

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      config.Byte `json:"shared_buffers"`
	EffectiveCacheSize config.Byte `json:"effective_cache_size"`
	WorkMem            config.Byte `json:"work_mem"`
	MaintenanceWorkMem config.Byte `json:"maintenance_work_mem"`
}

// Memory changes inspired by https://www.enterprisedb.com/postgres-tutorials/how-tune-postgresql-memory

// MaxMemoryPercent limits the maximum memory used
// for the profile when computing per connection buffers
var MaxMemoryPercent = map[string]float32{
	profile.Web:     0.25,
	profile.OLTP:    0.35,
	profile.DW:      0.50,
	profile.Mixed:   0.2,
	profile.Desktop: 0.1,
}

const (
	// SharedBufferPerc defines the percentage of ram
	// for the shared_buffers setting
	SharedBufferPerc = 0.25

	// EffectiveCacheSizePerc defines the percentage of ram
	// for the effective_cache_size setting - basically the
	// total_ram - shared_buffers
	EffectiveCacheSizePerc = 1 - SharedBufferPerc
)

// NewMemoryCfg creates a new Memory Configuration
func NewMemoryCfg(in config.Input) *MemoryCfg {

	return &MemoryCfg{
		SharedBuffers:      config.Byte(float32(in.TotalRAM) * SharedBufferPerc),
		EffectiveCacheSize: config.Byte(float32(in.TotalRAM) * EffectiveCacheSizePerc),
		WorkMem:            config.Byte(float32(in.TotalRAM) * MaxMemoryPercent[in.Profile] / float32(in.MaxConnections)),
		MaintenanceWorkMem: config.Byte(float32(in.TotalRAM) * 0.05),
	}
}
