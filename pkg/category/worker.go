package category

import (
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/profile"
)

// WorkerCfg is the main workers category
type WorkerCfg struct {
	MaxWorkerProcesses         int `json:"max_worker_processes" min_version:"9.4"`
	MaxParallelWorkerPerGather int `json:"max_parallel_workers_per_gather" min_version:"9.6"`
	MaxParallelWorkers         int `json:"max_parallel_workers" min_version:"10"`
}

// NewWorkerCfg creates a new Worker Configuration
func NewWorkerCfg(in input.Input) *WorkerCfg {
	// max_worker_processes: at least 8 (default), or CPU count
	maxWorkerProcesses := max(8, in.TotalCPU)

	// max_parallel_workers: at least 8, or CPU count (limited by max_worker_processes)
	maxParallelWorkers := max(8, in.TotalCPU)

	// max_parallel_workers_per_gather: varies by profile
	// OLTP/transactional workloads keep default (2)
	// DW/analytical workloads benefit from higher parallelism
	maxParallelWorkerPerGather := 2
	if in.Profile == profile.DW {
		// DW: use half of CPU cores, limited by max_parallel_workers
		maxParallelWorkerPerGather = min(in.TotalCPU/2, maxParallelWorkers)
		// Ensure at least 2
		if maxParallelWorkerPerGather < 2 {
			maxParallelWorkerPerGather = 2
		}
	}

	return &WorkerCfg{
		MaxWorkerProcesses:         maxWorkerProcesses,
		MaxParallelWorkerPerGather: maxParallelWorkerPerGather,
		MaxParallelWorkers:         maxParallelWorkers,
	}
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
