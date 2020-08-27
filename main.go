package main

import (
	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/compute"
)

func main() {

	// func NewInput(os string, arch string, totalRAM int, profile string, diskType string, maxConnections int, postgresVersion float32) *Input {
	in := compute.NewInput("linux", "x86_64", 64*compute.GB, "WEB", "SSD", 100, 12.4)

	_, out, err := compute.Compute(*in)

	if err != nil {
		panic(err)
	}

	spew.Dump(out)

}
