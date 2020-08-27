package main

import (
	"encoding/json"
	"fmt"

	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/compute"
	"github.com/pgconfig/api/pkg/config"
)

func main() {

	_, out, err := compute.Compute(
		*config.NewInput("linux", "x86_64", 64*config.GB, "WEB", "SSD", 100, 12.4))

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
