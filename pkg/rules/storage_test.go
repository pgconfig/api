package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
)

func Test_computeStorage(t *testing.T) {
	in := fakeInput()
	in.DiskType = "SSD"
	outSSD, _ := computeStorage(in, category.NewExportCfg(*in))
	in.DiskType = "SAN"
	outSAN, _ := computeStorage(in, category.NewExportCfg(*in))
	in.DiskType = "HDD"
	outHDD, _ := computeStorage(in, category.NewExportCfg(*in))

	if outSSD.Storage.RandomPageCost > 1.1 || outSAN.Storage.RandomPageCost > 1.1 {
		t.Error("should use lower values for random_page_cost on both SSD and SAN")
	}

	if outSSD.Storage.EffectiveIOConcurrency < 200 || outSAN.Storage.EffectiveIOConcurrency < 300 {
		t.Error("should use higher values for effective_io_concurrency on both SSD and SAN")
	}

	if outHDD.Storage.EffectiveIOConcurrency > 2 {
		t.Error("should use lower values for effective_io_concurrency on HDD drives")
	}
}
