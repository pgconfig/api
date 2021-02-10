package v1

import (
	"bytes"
	"fmt"
	"strconv"
	"strings"
	"unicode"

	"github.com/gofiber/fiber/v2"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/docs"
	"github.com/pgconfig/api/pkg/rules"
)

// GetConfig is a function to that computes the input and suggests a tuning configuration
// @Summary Get Configuration
// @Description computes the input and suggests a tuning configuration
// @Accept json
// @Produce json
// @Param pg_version query string false "PostgreSQL Version" default(13)
// @Param total_ram query string false "Total dedicated memory to PostgreSQL" default(2GB)
// @Param max_connections query integer false "Total expected number of connections" default(100)
// @Param environment_name query string false "Application profile of the server" Enums(WEB,OLTP,DW,Mixed,Desktop) default(WEB)
// @Param os_type query string false "Type of operating system used" Enums(linux,windows,unix) default(linux)
// @Param arch query string false "server architecture" Enums(386,amd64,arm,arm64) default(amd64)
// @Param drive_type query string false "default storage type" Enums(HDD,SSD,SAN) default(HDD)
// @Param cpus query integer false "Total CPUs available" default(2)
// @Param format query string false "Output format" Enums(json,alter_system,conf) default(json)
// @Param show_doc query string false "Show Documentation args" Enums(true,false) default(false)
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-config [get]
func GetConfig(c *fiber.Ctx) error {

	cpuCount, err := strconv.Atoi(c.Query("cpus", "2"))

	if err != nil {
		return err
	}
	maxConn, err := strconv.Atoi(c.Query("max_connections", "100"))

	if err != nil {
		return err
	}
	pgVersion, err := strconv.ParseFloat(c.Query("pg_version", defaultPgVersion), 32)

	if err != nil {
		return err
	}

	input := *config.NewInput(
		c.Query("os_type", "linux"),
		c.Query("arch", "amd64"),
		parseRAM(strings.ToUpper(c.Query("total_ram", "2GB"))),
		cpuCount,
		c.Query("environment_name", "WEB"),
		c.Query("drive_type", "HDD"),
		maxConn,
		float32(pgVersion),
	)

	tune, err := rules.Compute(input)

	if err != nil {
		return err
	}

	showDocs := c.Query("show_doc", "false") == "true"
	output := addDocTORules(docs.FormatVer(float32(pgVersion)), showDocs)

	return c.JSON(v1Reponse(c, setValues(output, tune)))
}

func setValues(output []outputCategory, tune *category.ExportCfg) []outputCategory {

	for c := 0; c < len(output); c++ {
		for p := 0; p < len(output[c].Parameters); p++ {

			switch output[c].Parameters[p].Name {

			// Memory Config
			case "shared_buffers":
				output[c].Parameters[p].Value = tune.Memory.SharedBuffers.String()
			case "effective_cache_size":
				output[c].Parameters[p].Value = tune.Memory.EffectiveCacheSize.String()
			case "work_mem":
				output[c].Parameters[p].Value = tune.Memory.WorkMem.String()
			case "maintenance_work_mem":
				output[c].Parameters[p].Value = tune.Memory.MaintenanceWorkMem.String()

			// Checkpoint Config
			case "min_wal_size":
				output[c].Parameters[p].Value = tune.Checkpoint.MinWALSize.String()
			case "max_wal_size":
				output[c].Parameters[p].Value = tune.Checkpoint.MaxWALSize.String()
			case "checkpoint_completion_target":
				output[c].Parameters[p].Value = fmt.Sprintf("%.1f", tune.Checkpoint.CheckpointCompletionTarget)
			case "wal_buffers":
				output[c].Parameters[p].Value = tune.Checkpoint.WALBuffers.String()
			case "checkpoint_segments":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Checkpoint.CheckpointSegments)

			// Network config
			case "listen_addresses":
				output[c].Parameters[p].Value = tune.Network.ListenAddresses
			case "max_connections":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Network.MaxConnections)

			// Storage config
			case "random_page_cost":
				output[c].Parameters[p].Value = fmt.Sprintf("%.1f", tune.Storage.RandomPageCost)
			case "effective_io_concurrency":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Storage.EffectiveIOConcurrency)

			// workers
			case "max_parallel_workers":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Worker.MaxParallelWorkers)
			case "max_parallel_workers_per_gather":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Worker.MaxParallelWorkerPerGather)
			case "max_worker_processes":
				output[c].Parameters[p].Value = fmt.Sprintf("%d", tune.Worker.MaxWorkerProcesses)
			}

			if output[c].Parameters[p].Value == "" {
				output[c].Parameters[p] = nil
			}
		}
	}

	return output
}

func parseRAM(compared string) config.Byte {

	val := extractNumber([]rune(compared))

	switch {
	case strings.HasSuffix(compared, "KB"):
		return val * config.KB
	case strings.HasSuffix(compared, "MB"):
		return val * config.MB
	case strings.HasSuffix(compared, "GB"):
		return val * config.GB
	case strings.HasSuffix(compared, "TB"):
		return val * config.TB
	default:
		return val
	}
}

func extractNumber(val []rune) config.Byte {

	var b bytes.Buffer

	for i := 0; i < len(val); i++ {
		if unicode.IsNumber(val[i]) {
			b.WriteRune(val[i])
		}
	}

	num, err := strconv.Atoi(b.String())

	if err != nil {
		panic(err)
	}

	return config.Byte(num)
}
