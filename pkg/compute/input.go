package compute

// Input is bla
type Input struct {
	OS              string
	Arch            string
	TotalRAM        int
	Profile         string
	DiskType        string
	MaxConnections  int
	PostgresVersion float32
}

// NewInput creates a Input
func NewInput(os string, arch string, totalRAM int, profile string, diskType string, maxConnections int, postgresVersion float32) *Input {
	return &Input{
		OS:              os,
		Arch:            arch,
		TotalRAM:        totalRAM,
		Profile:         profile,
		DiskType:        diskType,
		MaxConnections:  maxConnections,
		PostgresVersion: postgresVersion,
	}
}
