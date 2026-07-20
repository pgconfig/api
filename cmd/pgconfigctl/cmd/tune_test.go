package cmd

import (
	"runtime"
	"strings"
	"testing"

	"github.com/pgconfig/api/pkg/defaults"
	"github.com/pgconfig/api/pkg/format"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	"github.com/pgconfig/api/pkg/rules"
)

func TestTunePreservesLegacyOutput(t *testing.T) {
	tests := []struct {
		name            string
		input           input.Input
		includePgbadger bool
		logFormat       string
	}{
		{
			name: "explicit inputs",
			input: *input.NewInput(
				"linux", "amd64", 16*bytes.GB, 8, profile.OLTP, "SSD", 200, 16,
			),
			includePgbadger: true,
			logFormat:       "jsonlog",
		},
		{
			name: "flag defaults",
			input: *input.NewInput(
				"linux", "amd64", 8*bytes.GB, 4, profile.Web, "SSD", 100, defaults.PGVersionF,
			),
			logFormat: "csvlog",
		},
		{
			name: "autodetected resources",
			input: *input.NewInput(
				runtime.GOOS, runtime.GOARCH, totalRAM, runtime.NumCPU(), profile.Web, "SSD", 100, defaults.PGVersionF,
			),
			logFormat: "jsonlog",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			legacy, err := rules.Compute(tt.input)
			if err != nil {
				t.Fatalf("compute legacy recommendations: %v", err)
			}

			for _, outputFormat := range format.AllExportFormats {
				t.Run(string(outputFormat), func(t *testing.T) {
					want := format.ExportConf(
						outputFormat,
						legacy.ToSlice(tt.input.PostgresVersion, tt.includePgbadger, tt.logFormat),
						tt.input.PostgresVersion,
						nil,
					)
					got, err := tune(tt.input, outputFormat, tt.includePgbadger, tt.logFormat)
					if err != nil {
						t.Fatalf("tune: %v", err)
					}
					if got != want {
						t.Errorf("output changed\nwant:\n%s\ngot:\n%s", want, got)
					}
				})
			}
		})
	}
}

func TestTuneKeepsLegacyListenAddresses(t *testing.T) {
	in := *input.NewInput("linux", "amd64", 8*bytes.GB, 4, profile.Web, "SSD", 100, 16)

	output, err := tune(in, format.Config, false, "csvlog")
	if err != nil {
		t.Fatalf("tune: %v", err)
	}
	if !strings.Contains(output, "listen_addresses") {
		t.Fatal("legacy CLI output omitted listen_addresses")
	}
}
