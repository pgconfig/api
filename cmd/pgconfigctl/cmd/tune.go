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
	"os"
	"runtime"

	"github.com/davecgh/go-spew/spew"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/rules"
	"github.com/spf13/cobra"

	"github.com/mackerelio/go-osstat/memory"
)

var (
	version        float32
	osName         string
	arch           string
	totalCPU       int
	totalRAM       uint64
	maxConnections int
	diskType       string
	profile        string
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
				totalCPU,
				int(totalRAM),
				profile,
				diskType,
				100,
				version))

		if err != nil {
			panic(err)
		}

		spew.Dump(out)

		fmt.Println("\n=== JSON OUTPUT ================")
		b, err := json.MarshalIndent(out, "", "  ")
		if err != nil {
			panic(err)
		}
		fmt.Println(string(b))
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
	tuneCmd.PersistentFlags().Float32VarP(&version, "version", "", 12.4, "PostgreSQL Version")
	tuneCmd.PersistentFlags().IntVarP(&totalCPU, "cpus", "c", runtime.NumCPU(), "Total CPU cores")
	tuneCmd.PersistentFlags().Uint64VarP(&totalRAM, "ram", "", memory.Total, "Total Memory in bytes")
	tuneCmd.PersistentFlags().IntVarP(&maxConnections, "max-connections", "M", 100, "Max expected connections")
}
