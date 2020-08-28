package category

import "github.com/pgconfig/api/pkg/config"

// WorkerCfg is the main workers category
type WorkerCfg struct {
	MaxWorkerProcesses         int `json:"max_worker_processes,"`
	MaxParallelWorkerPerGather int `json:"max_parallel_workers_per_gather,omitempty"`
	MaxParallelWorkers         int `json:"max_parallel_workers,omitempty"`
}

// NewWorkerCfg creates a new Worker Configuration
func NewWorkerCfg(in config.Input) *WorkerCfg {
	return &WorkerCfg{
		MaxWorkerProcesses:         8, /* pg >= 9.4 */
		MaxParallelWorkerPerGather: 2, /* pg >= 9.6 */
		MaxParallelWorkers:         2, /* pg >= 10 */
	}
}
