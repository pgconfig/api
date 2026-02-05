package category

import (
	"testing"

	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

func TestNewWorkerCfg(t *testing.T) {
	tests := []struct {
		name                              string
		profile                           profile.Profile
		totalCPU                          int
		expectedMaxWorkerProcesses        int
		expectedMaxParallelWorkers        int
		expectedMaxParallelWorkerPerGather int
		description                       string
	}{
		{
			name:                              "Desktop with 4 cores",
			profile:                           profile.Desktop,
			totalCPU:                          4,
			expectedMaxWorkerProcesses:        8,
			expectedMaxParallelWorkers:        8,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "Small system uses minimum of 8 for worker processes",
		},
		{
			name:                              "Web with 8 cores",
			profile:                           profile.Web,
			totalCPU:                          8,
			expectedMaxWorkerProcesses:        8,
			expectedMaxParallelWorkers:        8,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "Web keeps default parallel workers per gather",
		},
		{
			name:                              "OLTP with 16 cores",
			profile:                           profile.OLTP,
			totalCPU:                          16,
			expectedMaxWorkerProcesses:        16,
			expectedMaxParallelWorkers:        16,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "OLTP scales workers with CPU but keeps per-gather at 2",
		},
		{
			name:                              "Mixed with 16 cores",
			profile:                           profile.Mixed,
			totalCPU:                          16,
			expectedMaxWorkerProcesses:        16,
			expectedMaxParallelWorkers:        16,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "Mixed workload uses default parallel workers per gather",
		},
		{
			name:                              "DW with 8 cores",
			profile:                           profile.DW,
			totalCPU:                          8,
			expectedMaxWorkerProcesses:        8,
			expectedMaxParallelWorkers:        8,
			expectedMaxParallelWorkerPerGather: 4,
			description:                       "DW uses CPU/2 for parallel workers per gather",
		},
		{
			name:                              "DW with 16 cores",
			profile:                           profile.DW,
			totalCPU:                          16,
			expectedMaxWorkerProcesses:        16,
			expectedMaxParallelWorkers:        16,
			expectedMaxParallelWorkerPerGather: 8,
			description:                       "DW scales parallel workers per gather with CPU",
		},
		{
			name:                              "DW with 32 cores",
			profile:                           profile.DW,
			totalCPU:                          32,
			expectedMaxWorkerProcesses:        32,
			expectedMaxParallelWorkers:        32,
			expectedMaxParallelWorkerPerGather: 16,
			description:                       "DW with many cores gets high parallelism",
		},
		{
			name:                              "DW with 2 cores ensures minimum",
			profile:                           profile.DW,
			totalCPU:                          2,
			expectedMaxWorkerProcesses:        8,
			expectedMaxParallelWorkers:        8,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "DW with few cores still gets minimum values",
		},
		{
			name:                              "Large system with 64 cores",
			profile:                           profile.OLTP,
			totalCPU:                          64,
			expectedMaxWorkerProcesses:        64,
			expectedMaxParallelWorkers:        64,
			expectedMaxParallelWorkerPerGather: 2,
			description:                       "Large OLTP system scales workers but not per-gather",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			in := input.Input{
				OS:              "linux",
				Arch:            "amd64",
				Profile:         tt.profile,
				TotalCPU:        tt.totalCPU,
				TotalRAM:        16 * bytes.GB,
				MaxConnections:  100,
				DiskType:        "SSD",
				PostgresVersion: 16.0,
			}

			cfg := NewWorkerCfg(in)

			if cfg.MaxWorkerProcesses != tt.expectedMaxWorkerProcesses {
				t.Errorf("%s: expected max_worker_processes = %d, got %d",
					tt.description, tt.expectedMaxWorkerProcesses, cfg.MaxWorkerProcesses)
			}

			if cfg.MaxParallelWorkers != tt.expectedMaxParallelWorkers {
				t.Errorf("%s: expected max_parallel_workers = %d, got %d",
					tt.description, tt.expectedMaxParallelWorkers, cfg.MaxParallelWorkers)
			}

			if cfg.MaxParallelWorkerPerGather != tt.expectedMaxParallelWorkerPerGather {
				t.Errorf("%s: expected max_parallel_workers_per_gather = %d, got %d",
					tt.description, tt.expectedMaxParallelWorkerPerGather, cfg.MaxParallelWorkerPerGather)
			}
		})
	}
}
