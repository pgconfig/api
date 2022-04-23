package cmd

/*
Copyright © 2020 Sebastian Webber <sebastian@pgconfig.org>

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

import (
	"fmt"
	"os"
	"runtime"

	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/defaults"
	"github.com/pgconfig/api/pkg/format"
	"github.com/pgconfig/api/pkg/profile"
	"github.com/pgconfig/api/pkg/rules"
	"github.com/spf13/cobra"

	"github.com/mackerelio/go-osstat/memory"
)

var (
	pgVersion       float32
	osName          string
	arch            string
	totalCPU        int
	totalRAM        config.Byte
	maxConnections  int
	diskType        string
	profileName     profile.Profile
	outputFormat    format.ExportFormat
	includePgbadger bool
	logFormat       string
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
				totalRAM,
				totalCPU,
				profileName,
				diskType,
				maxConnections,
				pgVersion))

		if err != nil {
			panic(err)
		}

		data := out.ToSlice(pgVersion, includePgbadger, logFormat)
		fmt.Println(format.ExportConf(outputFormat, data, pgVersion, ""))
	},
}

func init() {

	memory, err := memory.Get()
	if err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		return
	}

	totalRAM = config.Byte(memory.Total)
	profileName = profile.Web
	outputFormat = format.Config

	rootCmd.AddCommand(tuneCmd)

	tuneCmd.PersistentFlags().StringVarP(&osName, "os", "", runtime.GOOS, "Operating system")
	tuneCmd.PersistentFlags().StringVarP(&arch, "arch", "", runtime.GOARCH, "PostgreSQL Version")
	tuneCmd.PersistentFlags().StringVarP(&diskType, "disk-type", "D", "SSD", "Disk type (possible values are SSD, HDD and SAN)")
	tuneCmd.PersistentFlags().Float32VarP(&pgVersion, "version", "", defaults.PGVersionF, "PostgreSQL Version")
	tuneCmd.PersistentFlags().IntVarP(&totalCPU, "cpus", "c", runtime.NumCPU(), "Total CPU cores")
	tuneCmd.PersistentFlags().MarkDeprecated("env-name", "please use --profile instead")
	tuneCmd.PersistentFlags().IntVarP(&maxConnections, "max-connections", "M", 100, "Max expected connections")
	tuneCmd.PersistentFlags().BoolVarP(&includePgbadger, "include-pgbadger", "B", false, "Include pgbadger params?")
	tuneCmd.PersistentFlags().StringVarP(&logFormat, "log-format", "L", "csvlog", "Default log format")

	tuneCmd.PersistentFlags().VarP(&totalRAM, "ram", "", "Total Memory in bytes")
	tuneCmd.PersistentFlags().Lookup("ram").DefValue = config.FormatBytes(totalRAM)
	tuneCmd.PersistentFlags().VarP(&profileName, "profile", "", "Tuning profile")
	tuneCmd.PersistentFlags().Lookup("profile").DefValue = profileName.String()
	tuneCmd.PersistentFlags().VarP(&outputFormat, "format", "F", "config file format (possible values are unix, alter-system, stackgres, and json) - file extension also work (conf, sql, json, yaml)")
	tuneCmd.PersistentFlags().Lookup("format").DefValue = outputFormat.String()
	tuneCmd.PersistentFlags().Parse(os.Args[1:])
}
