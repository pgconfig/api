package main

import (
	"flag"
	"fmt"
	"log"
	"os"

	"github.com/pgconfig/api/pkg/docs"
	"gopkg.in/yaml.v2"
)

var (
	targetFile string
)

func init() {
	flag.StringVar(&targetFile, "target-file", "/home/seba/projetos/github.com/pgconfig/api/pg-docs.yml", "default target doc file")
	flag.Parse()
}

func saveFile(file docs.DocFile) error {

	f, err := os.Create(targetFile)

	if err != nil {
		return fmt.Errorf("could not create file: %w", err)
	}
	defer f.Close()

	d, err := yaml.Marshal(&file)
	if err != nil {
		return fmt.Errorf("could not marshal file: %w", err)
	}
	fmt.Fprintf(f, "---\n%s\n", string(d))

	return nil
}

func main() {

	file := docs.DocFile{
		Documentation: make(map[string]docs.Doc),
	}

	allVersions := []float32{
		9.1,
		9.2,
		9.3,
		9.4,
		9.5,
		9.6,
		10.0,
		11.0,
		12.0,
		13.0,
	}

	allParams := []string{
		"shared_buffers",
		"effective_cache_size",
		"work_mem",
		"maintenance_work_mem",
		"min_wal_size",
		"max_wal_size",
		"checkpoint_segments",
		"checkpoint_completion_target",
		"wal_buffers",
		"listen_addresses",
		"max_connections",
		"random_page_cost",
		"effective_io_concurrency",
		"max_worker_processes",
		"max_parallel_workers_per_gather",
		"max_parallel_workers",
	}

	for _, ver := range allVersions {
		file.Documentation[docs.FormatVer(ver)] = make(docs.Doc)
	}

	for _, param := range allParams {
		for _, ver := range allVersions {

			a, err := docs.Get(param, ver)

			fmt.Printf("Processing %s from version %s... ", param, docs.FormatVer(ver))
			// 404 unsupported
			if err != nil {
				fmt.Println("SKIPPED")
				continue
			}

			fmt.Println()

			file.Documentation[docs.FormatVer(ver)][param] = a
		}
	}

	err := saveFile(file)

	if err != nil {
		log.Printf("Could not save file: %v", err)
	}

}
