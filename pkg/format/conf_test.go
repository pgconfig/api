package format

import (
	"strings"
	"testing"

	"github.com/andreyvit/diff"
	"github.com/pgconfig/api/pkg/category"
)

var sliceConfSample = []category.SliceOutput{
	category.SliceOutput{
		Name:        "memory_related",
		Description: "Memory Configuration",
		Parameters: []category.ParamSliceOutput{
			category.ParamSliceOutput{Name: "shared_buffers", Value: "23GB", Format: "Byte"},
			category.ParamSliceOutput{Name: "effective_cache_size", Value: "70GB", Format: "Byte"},
			category.ParamSliceOutput{Name: "work_mem", Value: "965MB", Format: "Byte"},
			category.ParamSliceOutput{Name: "maintenance_work_mem", Value: "5GB", Format: "Byte"}}},
	category.SliceOutput{
		Name:        "checkpoint_related",
		Description: "Checkpoint Related Configuration",
		Parameters: []category.ParamSliceOutput{
			category.ParamSliceOutput{Name: "min_wal_size", Value: "2GB", Format: "Byte"},
			category.ParamSliceOutput{Name: "max_wal_size", Value: "3GB", Format: "Byte"},
			category.ParamSliceOutput{Name: "checkpoint_completion_target", Value: "0.5", Format: "float32"},
			category.ParamSliceOutput{Name: "wal_buffers", Value: "-1", Format: "Byte"},
		}},
	category.SliceOutput{
		Name:        "network_related",
		Description: "Network Related Configuration",
		Parameters: []category.ParamSliceOutput{
			category.ParamSliceOutput{Name: "listen_addresses", Value: "*", Format: "string"},
			category.ParamSliceOutput{Name: "max_connections", Value: "100", Format: "int"}}},
	category.SliceOutput{
		Name:        "storage_type",
		Description: "Storage Configuration",
		Parameters: []category.ParamSliceOutput{
			category.ParamSliceOutput{Name: "random_page_cost", Value: "4.0", Format: "float32"},
			category.ParamSliceOutput{Name: "effective_io_concurrency", Value: "1", Format: "int"}}},
	category.SliceOutput{
		Name:        "worker_related",
		Description: "Worker Processes Configuration",
		Parameters: []category.ParamSliceOutput{
			category.ParamSliceOutput{Name: "max_worker_processes", Value: "8", Format: "int"},
			category.ParamSliceOutput{Name: "max_parallel_workers_per_gather", Value: "2", Format: "int"},
			category.ParamSliceOutput{Name: "max_parallel_workers", Value: "2", Format: "int"}}}}

func TestConfigFile(t *testing.T) {
	sample := `
# Memory Configuration
shared_buffers = 23GB
effective_cache_size = 70GB
work_mem = 965MB
maintenance_work_mem = 5GB

# Checkpoint Related Configuration
min_wal_size = 2GB
max_wal_size = 3GB
checkpoint_completion_target = 0.5
wal_buffers = -1

# Network Related Configuration
listen_addresses = '*'
max_connections = 100

# Storage Configuration
random_page_cost = 4.0
effective_io_concurrency = 1

# Worker Processes Configuration
max_worker_processes = 8
max_parallel_workers_per_gather = 2
max_parallel_workers = 2
`

	out := ConfigFile(sliceConfSample)

	if a, e := strings.TrimSpace(sample), strings.TrimSpace(out); a != e {
		t.Errorf("Result not as expected:\n%v", diff.LineDiff(e, a))
	}

}
