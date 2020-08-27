package category

import "github.com/pgconfig/api/pkg/config"

// StorageCfg is the main memory category
type StorageCfg struct {
	RandomPageCost         float32 `json:"random_page_cost"`
	EffectiveIOConcurrency int     `json:"effective_io_concurrency"`
}

// NewStorageCfg creates a new Storage Configuration
//
// both random_page_cost and effective_io_concurrency are set
// with the default value.
func NewStorageCfg(in config.Input) *StorageCfg {
	return &StorageCfg{
		RandomPageCost:         4.0,
		EffectiveIOConcurrency: 1,
	}
}
