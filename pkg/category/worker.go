package category

import "github.com/pgconfig/api/pkg/input"

// WorkerCfg is the main workers category
type WorkerCfg struct {
	MaxWorkerProcesses         int `json:"max_worker_processes" min_version:"9.4"`
	MaxParallelWorkerPerGather int `json:"max_parallel_workers_per_gather" min_version:"9.6"`
	MaxParallelWorkers         int `json:"max_parallel_workers" min_version:"10"`
}

// NewWorkerCfg creates a new Worker Configuration
func NewWorkerCfg(in input.Input) *WorkerCfg {
	return &WorkerCfg{
		MaxWorkerProcesses:         8, /* pg >= 9.4 */
		MaxParallelWorkerPerGather: 2, /* pg >= 9.6 */
		MaxParallelWorkers:         2, /* pg >= 10 */
	}
}
