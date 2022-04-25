package category

import (
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      bytes.Byte `json:"shared_buffers"`
	EffectiveCacheSize bytes.Byte `json:"effective_cache_size"`
	WorkMem            bytes.Byte `json:"work_mem"`
	MaintenanceWorkMem bytes.Byte `json:"maintenance_work_mem"`
}

// Memory changes inspired by https://www.enterprisedb.com/postgres-tutorials/how-tune-postgresql-memory

// MaxMemoryBuffersPercent limits the maximum memory used
// for the profile when computing per connection buffers
var MaxMemoryBuffersPercent = map[profile.Profile]float32{
	profile.Web:     0.25,
	profile.OLTP:    0.35,
	profile.DW:      0.50,
	profile.Mixed:   0.2,
	profile.Desktop: 0.1,
}

// MaxMemoryProfilePercent limits the max memory used
// in the profile.
var MaxMemoryProfilePercent = map[profile.Profile]float32{
	profile.Web:     1,
	profile.OLTP:    1,
	profile.DW:      1,
	profile.Mixed:   0.5,
	profile.Desktop: 0.2,
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
func NewMemoryCfg(in input.Input) *MemoryCfg {

	totalRAM := float32(in.TotalRAM) * MaxMemoryProfilePercent[in.Profile]

	return &MemoryCfg{
		SharedBuffers:      bytes.Byte(totalRAM * SharedBufferPerc),
		EffectiveCacheSize: bytes.Byte(totalRAM * EffectiveCacheSizePerc),
		WorkMem:            bytes.Byte(totalRAM * MaxMemoryBuffersPercent[in.Profile] / float32(in.MaxConnections)),
		MaintenanceWorkMem: bytes.Byte(totalRAM * 0.05),
	}
}
