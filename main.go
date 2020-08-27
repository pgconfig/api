package main

import (
	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/compute"
	"github.com/pgconfig/api/pkg/config"
)

func main() {

	_, out, err := compute.Compute(
		*config.NewInput("linux", "x86_64", 64*compute.GB, "WEB", "SSD", 100, 12.4))

	if err != nil {
		panic(err)
	}

	spew.Dump(out)

}
