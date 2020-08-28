package main

import (
	"encoding/json"
	"flag"
	"fmt"

	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/compute"
	"github.com/pgconfig/api/pkg/config"
)

var pgVersion float64

func init() {
	flag.Float64Var(&pgVersion, "version", 12.0, "PostgreSQL Version")
	flag.Parse()
}

func main() {

	_, out, err := compute.Compute(
		*config.NewInput("linux", "x86_64", 64*config.GB, "WEB", "SSD", 100, float32(pgVersion)))

	if err != nil {
		panic(err)
	}

	spew.Dump(out)

	fmt.Println("\n=== JSON OUTPUT ================")
	printJSON(out)

}

func printJSON(output *category.ExportCfg) {

	b, err := json.MarshalIndent(output, "", "  ")
	if err != nil {
		panic(err)
	}
	fmt.Println(string(b))
}
