package main

import (
	"flag"
	"fmt"
	"github.com/charmbracelet/log"
	"os"
	"sync"

	"gopkg.in/yaml.v2"
)

var (
	targetFile string
	limiter    chan int
	file       DocFile
	mu         sync.Mutex
)

func init() {
	currDir, _ := os.Getwd()
	flag.StringVar(&targetFile, "target-file", fmt.Sprintf("%s/pg-docs.yml", currDir), "default target doc file")

	maxJobs := 5
	flag.IntVar(&maxJobs, "jobs", maxJobs, "max jobs")

	limiter = make(chan int, maxJobs)
	flag.Parse()
}

func saveFile(file DocFile) error {

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

func updateDoc(ver float32, param string, parsed ParamDoc) {
	mu.Lock()
	defer mu.Unlock()
	file.Documentation[FormatVer(ver)][param] = parsed
}

func main() {
	file = DocFile{
		Documentation: make(map[string]Doc),
	}

	allVersions := supportedVersions

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
		"maintenance_io_concurrency",
		"io_method",
		"io_workers",
		"io_max_combine_limit",
		"io_max_concurrency",
		"file_copy_method",
		"max_worker_processes",
		"max_parallel_workers_per_gather",
		"max_parallel_workers",
	}

	for _, ver := range allVersions {
		file.Documentation[FormatVer(ver)] = make(Doc)
	}
	var wg sync.WaitGroup
	for _, param := range allParams {
		for _, ver := range allVersions {
			wg.Add(1)
			limiter <- 1

			go processParam(param, ver, &wg)

		}
	}

	wg.Wait()

	err := saveFile(file)

	if err != nil {
		log.Errorf("Could not save file: %v", err)
	}

}

func processParam(param string, ver float32, wg *sync.WaitGroup) {
	defer func() {
		wg.Done()
		<-limiter
	}()

	parsed, err := Get(param, ver)

	// 404 means unsupported
	if err != nil {
		fmt.Printf("Processing %s from version %s... SKIPPED\n", param, FormatVer(ver))
		return
	}

	fmt.Printf("Processing %s from version %s... \n", param, FormatVer(ver))

	updateDoc(ver, param, parsed)
}
