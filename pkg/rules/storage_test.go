package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
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

	// maintenance_io_concurrency should match effective_io_concurrency
	if outSSD.Storage.MaintenanceIOConcurrency != outSSD.Storage.EffectiveIOConcurrency {
		t.Error("maintenance_io_concurrency should match effective_io_concurrency for SSD")
	}
	if outSAN.Storage.MaintenanceIOConcurrency != outSAN.Storage.EffectiveIOConcurrency {
		t.Error("maintenance_io_concurrency should match effective_io_concurrency for SAN")
	}
	if outHDD.Storage.MaintenanceIOConcurrency != outHDD.Storage.EffectiveIOConcurrency {
		t.Error("maintenance_io_concurrency should match effective_io_concurrency for HDD")
	}
}

func Test_computeStorageRandomPageCost(t *testing.T) {
	tests := []struct {
		name                  string
		profile               profile.Profile
		diskType              string
		expectedRandomPageCost float32
		description           string
	}{
		{
			name:                  "DW profile with SSD gets higher random_page_cost",
			profile:               profile.DW,
			diskType:              "SSD",
			expectedRandomPageCost: 1.8,
			description:           "DW analytical queries favor sequential scans",
		},
		{
			name:                  "DW profile with SAN gets higher random_page_cost",
			profile:               profile.DW,
			diskType:              "SAN",
			expectedRandomPageCost: 1.8,
			description:           "DW analytical queries favor sequential scans on SAN too",
		},
		{
			name:                  "DW profile with HDD keeps default",
			profile:               profile.DW,
			diskType:              "HDD",
			expectedRandomPageCost: 4.0,
			description:           "HDD uses PostgreSQL default",
		},
		{
			name:                  "OLTP profile with SSD gets low random_page_cost",
			profile:               profile.OLTP,
			diskType:              "SSD",
			expectedRandomPageCost: 1.1,
			description:           "OLTP favors index scans",
		},
		{
			name:                  "Web profile with SSD gets low random_page_cost",
			profile:               profile.Web,
			diskType:              "SSD",
			expectedRandomPageCost: 1.1,
			description:           "Web workload favors index scans",
		},
		{
			name:                  "Mixed profile with SSD gets low random_page_cost",
			profile:               profile.Mixed,
			diskType:              "SSD",
			expectedRandomPageCost: 1.1,
			description:           "Mixed workload uses general SSD value",
		},
		{
			name:                  "Desktop profile with SSD gets low random_page_cost",
			profile:               profile.Desktop,
			diskType:              "SSD",
			expectedRandomPageCost: 1.1,
			description:           "Desktop uses general SSD value",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			in := input.Input{
				OS:              "linux",
				Arch:            "amd64",
				Profile:         tt.profile,
				DiskType:        tt.diskType,
				TotalRAM:        100 * bytes.GB,
				MaxConnections:  100,
				TotalCPU:        8,
				PostgresVersion: 16.0,
			}

			out, err := computeStorage(&in, category.NewExportCfg(in))
			if err != nil {
				t.Fatalf("computeStorage failed: %v", err)
			}

			if out.Storage.RandomPageCost != tt.expectedRandomPageCost {
				t.Errorf("%s: expected random_page_cost = %v, got %v",
					tt.description, tt.expectedRandomPageCost, out.Storage.RandomPageCost)
			}
		})
	}
}
