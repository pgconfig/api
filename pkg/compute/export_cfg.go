package compute

type ExportCfg struct {
	Memory *MemoryCfg
}

func NewExportCfg(in Input) *ExportCfg {
	return &ExportCfg{
		Memory: NewMemoryCfg(in),
	}
}
