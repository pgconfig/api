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

func TestComputeWalSizes(t *testing.T) {
	tests := []struct {
		name          string
		profile       profile.Profile
		totalRAM      bytes.Byte
		expectedMinWAL bytes.Byte
		expectedMaxWAL bytes.Byte
		description   string
	}{
		{
			name:          "DW profile gets large WAL sizes",
			profile:       profile.DW,
			totalRAM:      100 * bytes.GB,
			expectedMinWAL: 4 * bytes.GB,
			expectedMaxWAL: 16 * bytes.GB,
			description:   "Data Warehouse is write-heavy with batch jobs",
		},
		{
			name:          "OLTP profile gets medium-large WAL sizes",
			profile:       profile.OLTP,
			totalRAM:      100 * bytes.GB,
			expectedMinWAL: 2 * bytes.GB,
			expectedMaxWAL: 8 * bytes.GB,
			description:   "OLTP has frequent transactions",
		},
		{
			name:          "Web profile gets moderate WAL sizes",
			profile:       profile.Web,
			totalRAM:      100 * bytes.GB,
			expectedMinWAL: 1 * bytes.GB,
			expectedMaxWAL: 4 * bytes.GB,
			description:   "Web has moderate writes",
		},
		{
			name:          "Mixed profile gets balanced WAL sizes",
			profile:       profile.Mixed,
			totalRAM:      100 * bytes.GB,
			expectedMinWAL: 2 * bytes.GB,
			expectedMaxWAL: 6 * bytes.GB,
			description:   "Mixed workload is balanced",
		},
		{
			name:          "Desktop profile gets small WAL sizes",
			profile:       profile.Desktop,
			totalRAM:      16 * bytes.GB,
			expectedMinWAL: 512 * bytes.MB,
			expectedMaxWAL: 2 * bytes.GB,
			description:   "Desktop has low activity",
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

			if out.Checkpoint.MinWALSize != tt.expectedMinWAL {
				t.Errorf("%s: expected min_wal_size = %v, got %v",
					tt.description, tt.expectedMinWAL, out.Checkpoint.MinWALSize)
			}

			if out.Checkpoint.MaxWALSize != tt.expectedMaxWAL {
				t.Errorf("%s: expected max_wal_size = %v, got %v",
					tt.description, tt.expectedMaxWAL, out.Checkpoint.MaxWALSize)
			}
		})
	}
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
