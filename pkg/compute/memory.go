package compute

// MemoryCfg is the main memory category
type MemoryCfg struct {
	SharedBuffers      int
	EffectiveCacheSize int
	WorkMem            int
	MaintenanceWorkMem int
}

func NewMemoryCfg(in Input) *MemoryCfg {
	return &MemoryCfg{
		SharedBuffers:      in.TotalRAM / 4,
		EffectiveCacheSize: (in.TotalRAM / 4) * 3,
		WorkMem:            (in.TotalRAM / in.MaxConnections),
		MaintenanceWorkMem: in.TotalRAM / 16,
	}
}

/*
	mem := MemoryCfg{}
	computeOS(mem)

*/
