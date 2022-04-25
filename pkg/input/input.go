package input

import (
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

// Input are all needed values to compute all parameters
type Input struct {
	OS              string          `json:"os"`
	Arch            string          `json:"arch"`
	TotalRAM        bytes.Byte      `json:"total_ram"`
	Profile         profile.Profile `json:"profile"`
	DiskType        string          `json:"disk_type"`
	MaxConnections  int             `json:"max_connections"`
	TotalCPU        int             `json:"total_cpu"`
	PostgresVersion float32         `json:"postgres_version"`
}

// NewInput creates a Input
func NewInput(os string, arch string, totalRAM bytes.Byte, totalCPU int, profile profile.Profile, diskType string, maxConnections int, postgresVersion float32) *Input {

	return &Input{
		OS:              os,
		Arch:            arch,
		TotalRAM:        totalRAM,
		TotalCPU:        totalCPU,
		Profile:         profile,
		DiskType:        diskType,
		MaxConnections:  maxConnections,
		PostgresVersion: postgresVersion,
	}
}
