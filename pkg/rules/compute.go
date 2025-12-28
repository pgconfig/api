package rules

import (
	"fmt"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
)

type rule func(*input.Input, *category.ExportCfg) (*category.ExportCfg, error)

var allRules = []rule{
	computeArch,
	computeOS,
	computeProfile,
	computeStorage,
	computeAIO,

	// computeVersion can remove values deppending on the version
	// to be sure that it will not break other rules, leave it at
	// the end.
	computeVersion,
}

// Compute evaluate all parameters
func Compute(in input.Input) (*category.ExportCfg, error) {
	var (
		out *category.ExportCfg
		err error
	)

	out = category.NewExportCfg(in)

	for _, rule := range allRules {

		out, err = rule(&in, out)

		if err != nil {
			return nil, fmt.Errorf("could not process rule: %w", err)
		}
	}

	return out, nil
}
