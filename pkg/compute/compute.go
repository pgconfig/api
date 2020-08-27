package compute

// Compute evaluate all parameters
func Compute(in Input) (*Input, *ExportCfg, error) {
	return computeOS(computeArch(&in, NewExportCfg(in), nil))
}
