package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

func TestCompute(t *testing.T) {
	t.SkipNow()
}

func TestComputeWalBuffers(t *testing.T) {
	tests := []struct {
		name             string
		profile          profile.Profile
		totalRAM         bytes.Byte
		expectedWalBuffers bytes.Byte
		description      string
	}{
		{
			name:             "DW profile always gets 64MB",
			profile:          profile.DW,
			totalRAM:         100 * bytes.GB,
			expectedWalBuffers: 64 * bytes.MB,
			description:      "Data Warehouse is write-heavy",
		},
		{
			name:             "DW profile with small RAM still gets 64MB",
			profile:          profile.DW,
			totalRAM:         8 * bytes.GB,
			expectedWalBuffers: 64 * bytes.MB,
			description:      "DW always uses 64MB regardless of RAM",
		},
		{
			name:             "OLTP with large shared_buffers gets 32MB",
			profile:          profile.OLTP,
			totalRAM:         40 * bytes.GB, // shared_buffers = 40GB * 0.25 = 10GB > 8GB
			expectedWalBuffers: 32 * bytes.MB,
			description:      "Large OLTP systems benefit from larger wal_buffers",
		},
		{
			name:             "OLTP with small shared_buffers uses auto-tune",
			profile:          profile.OLTP,
			totalRAM:         16 * bytes.GB, // shared_buffers = 16GB * 0.25 = 4GB < 8GB
			expectedWalBuffers: -1,
			description:      "Small OLTP systems use auto-tuning",
		},
		{
			name:             "Web profile uses auto-tune",
			profile:          profile.Web,
			totalRAM:         100 * bytes.GB,
			expectedWalBuffers: -1,
			description:      "Web workload uses default auto-tuning",
		},
		{
			name:             "Mixed profile uses auto-tune",
			profile:          profile.Mixed,
			totalRAM:         100 * bytes.GB,
			expectedWalBuffers: -1,
			description:      "Mixed workload uses default auto-tuning",
		},
		{
			name:             "Desktop profile uses auto-tune",
			profile:          profile.Desktop,
			totalRAM:         16 * bytes.GB,
			expectedWalBuffers: -1,
			description:      "Desktop workload uses default auto-tuning",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			in := input.Input{
				OS:              "linux",
				Arch:            "amd64",
				Profile:         tt.profile,
				TotalRAM:        tt.totalRAM,
				MaxConnections:  100,
				DiskType:        "ssd",
				TotalCPU:        8,
				PostgresVersion: 16.0,
			}

			out, err := Compute(in)
			if err != nil {
				t.Fatalf("Compute failed: %v", err)
			}

			if out.Checkpoint.WALBuffers != tt.expectedWalBuffers {
				t.Errorf("%s: expected wal_buffers = %v, got %v",
					tt.description, tt.expectedWalBuffers, out.Checkpoint.WALBuffers)
			}
		})
	}
}
