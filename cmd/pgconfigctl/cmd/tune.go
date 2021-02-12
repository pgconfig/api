/*
Copyright © 2020 Sebastian Webber <sebastian@swebber.me>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/
package cmd

import (
	"encoding/json"
	"fmt"
	"html/template"
	"os"
	"runtime"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/rules"
	"github.com/spf13/cobra"

	"github.com/mackerelio/go-osstat/memory"
)

var (
	pgVersion      float32
	osName         string
	arch           string
	totalCPU       int
	totalRAM       int64
	maxConnections int
	diskType       string
	profile        string
	format         string
)

// tuneCmd represents the tune command
var tuneCmd = &cobra.Command{
	Use:   "tune",
	Short: "Tunes your PostgreSQL server",
	Long:  `Uses your server info to compute the PostgreSQL tuning aiming to give you a get-start to tune your server.`,
	Run: func(cmd *cobra.Command, args []string) {

		out, err := rules.Compute(
			*config.NewInput(
				osName,
				arch,
				config.Byte(totalRAM),
				totalCPU,
				profile,
				diskType,
				maxConnections,
				pgVersion))

		if err != nil {
			panic(err)
		}

		switch format {
		case "json":
			b, err := json.MarshalIndent(out.ToSlice(), "", "  ")
			if err != nil {
				panic(err)
			}
			fmt.Println(string(b))
		case "conf", "unix":
			err = formatOut(confTempl, out)
			if err != nil {
				panic(err)
			}
		case "alter-system", "sql":
			err = formatOut(sqlTempl, out)
			if err != nil {
				panic(err)
			}
		default:
			fmt.Println("Invalid format")
			os.Exit(1)
		}

	},
}

func init() {

	memory, err := memory.Get()
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		return
	}

	rootCmd.AddCommand(tuneCmd)

	tuneCmd.PersistentFlags().StringVarP(&osName, "os", "", runtime.GOOS, "Operating system")
	tuneCmd.PersistentFlags().StringVarP(&arch, "arch", "", runtime.GOARCH, "PostgreSQL Version")
	tuneCmd.PersistentFlags().StringVarP(&diskType, "disk-type", "D", "SSD", "Disk type (possible values are SSD, HDD and SAN)")
	tuneCmd.PersistentFlags().StringVarP(&profile, "profile", "", "WEB", "Tuning profile (possible values are WEB, HDD and SAN)")
	tuneCmd.PersistentFlags().StringVarP(&format, "format", "", "conf", "config file format (possible values are unix, alter-system, and json) - file extension also work (conf, sql, json)")
	tuneCmd.PersistentFlags().Float32VarP(&pgVersion, "version", "", 12.4, "PostgreSQL Version")
	tuneCmd.PersistentFlags().IntVarP(&totalCPU, "cpus", "c", runtime.NumCPU(), "Total CPU cores")
	tuneCmd.PersistentFlags().Int64VarP(&totalRAM, "ram", "", int64(memory.Total), "Total Memory in bytes")
	tuneCmd.PersistentFlags().IntVarP(&maxConnections, "max-connections", "M", 100, "Max expected connections")
}

var (
	confTempl = `## Generated with pgconfigctl
{{ if .Memory }}
## Memory Configuration
{{ if .Memory.SharedBuffers }}shared_buffers = {{ .Memory.SharedBuffers }}{{ end }}
{{ if .Memory.EffectiveCacheSize }}effective_cache_size = {{ .Memory.EffectiveCacheSize }}{{ end }}
{{ if .Memory.WorkMem }}work_mem = {{ .Memory.WorkMem }}{{ end }}
{{ if .Memory.MaintenanceWorkMem }}maintenance_work_mem = {{ .Memory.MaintenanceWorkMem }}{{ end }}
{{end }}
{{ if .Checkpoint }}
## Checkpoint Related Configuration
{{ if .Checkpoint.MinWALSize }}min_wal_size = {{ .Checkpoint.MinWALSize }}{{- end }}
{{ if .Checkpoint.MaxWALSize }}max_wal_size = {{ .Checkpoint.MaxWALSize }}{{- end }}
{{ if .Checkpoint.CheckpointCompletionTarget }}checkpoint_completion_target = {{ .Checkpoint.CheckpointCompletionTarget }}{{- end }}
{{ if .Checkpoint.WALBuffers }}wal_buffers = {{ .Checkpoint.WALBuffers }}{{- end }}
{{ if .Checkpoint.CheckpointSegments }}checkpoint_segments = {{ .Checkpoint.CheckpointSegments }}{{- end }}
{{- end }}
{{ if .Network }}
## Network Related Configuration
{{ if .Network.ListenAddresses }}listen_addresses = {{ .Network.ListenAddresses }}{{- end }}
{{ if .Network.MaxConnections }}max_connections = {{ .Network.MaxConnections }}{{- end }}
{{- end }}
{{ if .Storage }}
## Storage Configuration
{{ if .Storage.RandomPageCost }}random_page_cost = {{ .Storage.RandomPageCost }}{{- end }}
{{ if .Storage.EffectiveIOConcurrency }}effective_io_concurrency = {{ .Storage.EffectiveIOConcurrency }}{{- end }}
{{- end }}
{{ if .Worker }}
## Worker Processes Configuration
{{ if .Worker.MaxWorkerProcesses }}max_worker_processes = {{ .Worker.MaxWorkerProcesses }}{{- end }}
{{ if .Worker.MaxParallelWorkerPerGather }}max_parallel_workers_per_gather = {{ .Worker.MaxParallelWorkerPerGather }}{{- end }}
{{ if .Worker.MaxParallelWorkers }}max_parallel_workers = {{ .Worker.MaxParallelWorkers }}{{- end }}
{{- end }}
`
	sqlTempl = `-- Generated with pgconfigctl
{{ if .Memory }}
-- Memory Configuration
{{ if .Memory.SharedBuffers }}ALTER SYSTEM SET shared_buffers TO '{{ .Memory.SharedBuffers }}';{{ end }}
{{ if .Memory.EffectiveCacheSize }}ALTER SYSTEM SET effective_cache_size TO '{{ .Memory.EffectiveCacheSize }}';{{ end }}
{{ if .Memory.WorkMem }}ALTER SYSTEM SET work_mem TO '{{ .Memory.WorkMem }}';{{ end }}
{{ if .Memory.MaintenanceWorkMem }}ALTER SYSTEM SET maintenance_work_mem TO '{{ .Memory.MaintenanceWorkMem }}';{{ end }}
{{end }}
{{ if .Checkpoint }}
-- Checkpoint Related Configuration
{{ if .Checkpoint.MinWALSize }}ALTER SYSTEM SET min_wal_size TO '{{ .Checkpoint.MinWALSize }}';{{- end }}
{{ if .Checkpoint.MaxWALSize }}ALTER SYSTEM SET max_wal_size TO '{{ .Checkpoint.MaxWALSize }}';{{- end }}
{{ if .Checkpoint.CheckpointCompletionTarget }}ALTER SYSTEM SET checkpoint_completion_target TO '{{ .Checkpoint.CheckpointCompletionTarget }}';{{- end }}
{{ if .Checkpoint.WALBuffers }}ALTER SYSTEM SET wal_buffers TO '{{ .Checkpoint.WALBuffers }}';{{- end }}
{{ if .Checkpoint.CheckpointSegments }}ALTER SYSTEM SET checkpoint_segments TO '{{ .Checkpoint.CheckpointSegments }}';{{- end }}
{{- end }}
{{ if .Network }}
-- Network Related Configuration
{{ if .Network.ListenAddresses }}ALTER SYSTEM SET listen_addresses TO '{{ .Network.ListenAddresses }}';{{- end }}
{{ if .Network.MaxConnections }}ALTER SYSTEM SET max_connections TO '{{ .Network.MaxConnections }}';{{- end }}
{{- end }}
{{ if .Storage }}
-- Storage Configuration
{{ if .Storage.RandomPageCost }}ALTER SYSTEM SET random_page_cost TO '{{ .Storage.RandomPageCost }}';{{- end }}
{{ if .Storage.EffectiveIOConcurrency }}ALTER SYSTEM SET effective_io_concurrency TO '{{ .Storage.EffectiveIOConcurrency }}';{{- end }}
{{- end }}
{{ if .Worker }}
-- Worker Processes Configuration
{{ if .Worker.MaxWorkerProcesses }}ALTER SYSTEM SET max_worker_processes TO '{{ .Worker.MaxWorkerProcesses }}';{{- end }}
{{ if .Worker.MaxParallelWorkerPerGather }}ALTER SYSTEM SET max_parallel_workers_per_gather TO '{{ .Worker.MaxParallelWorkerPerGather }}';{{- end }}
{{ if .Worker.MaxParallelWorkers }}ALTER SYSTEM SET max_parallel_workers TO '{{ .Worker.MaxParallelWorkers }}';{{- end }}
{{- end }}
`
)

func formatOut(strTemplate string, out *category.ExportCfg) error {

	tmpl, err := template.New("config-export").Parse(strTemplate)
	if err != nil {
		return fmt.Errorf("could not parse template: %w", err)
	}
	err = tmpl.Execute(os.Stdout, out)
	if err != nil {
		return fmt.Errorf("could not execute template: %w", err)
	}

	return nil
}
