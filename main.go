package main

import (
	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/compute"
)

func main() {

	_, out, err := compute.Compute(
		compute.NewInput("linux", "x86_64", 64*compute.GB, "WEB", "SSD", 100, 12.4)
	)

	if err != nil {
		panic(err)
	}

	spew.Dump(out)

}
