package category

import "github.com/pgconfig/api/pkg/input"

// StorageCfg is the main memory category
type StorageCfg struct {
	RandomPageCost           float32 `json:"random_page_cost"`
	EffectiveIOConcurrency   int     `json:"effective_io_concurrency"`
	MaintenanceIOConcurrency int     `json:"maintenance_io_concurrency" min_version:"13"`
	IOMethod                 string  `json:"io_method" min_version:"18"`
	IOWorkers                int     `json:"io_workers" min_version:"18"`
	IOMaxCombineLimit        int     `json:"io_max_combine_limit" min_version:"18"`
	IOMaxConcurrency         int     `json:"io_max_concurrency" min_version:"18"`
	FileCopyMethod           string  `json:"file_copy_method" min_version:"18"`
}

// NewStorageCfg creates a new Storage Configuration
//
// both random_page_cost and effective_io_concurrency are set
// with the default value.
func NewStorageCfg(in input.Input) *StorageCfg {
	return &StorageCfg{
		RandomPageCost:           4.0,
		EffectiveIOConcurrency:   1,
		MaintenanceIOConcurrency: 10,
		IOMethod:                 "worker",
		IOWorkers:                3,
		IOMaxCombineLimit:        16,
		IOMaxConcurrency:         64,
		FileCopyMethod:           "copy",
	}
}
