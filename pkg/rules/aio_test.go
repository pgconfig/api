package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	"github.com/stretchr/testify/assert"
)

func Test_computeAIO(t *testing.T) {
	type args struct {
		in  *input.Input
		cfg *category.ExportCfg
	}
	tests := []struct {
		name    string
		args    args
		want    *category.ExportCfg
		wantErr bool
	}{
		{
			name: "Desktop profile with 4 cores",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        8 * bytes.GB,
					TotalCPU:        4,
					Profile:         profile.Desktop,
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				cfg.Storage.IOWorkers = 2 // 4 * 0.1 = 0.4 -> ceil = 1, min 2 -> 2
				return cfg
			}(),
		},
		{
			name: "DW profile with 16 cores HDD",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        32 * bytes.GB,
					TotalCPU:        16,
					Profile:         profile.DW,
					DiskType:        "HDD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// factor 0.4 + 0.1 for HDD = 0.5, 16 * 0.5 = 8, max workers = totalCPU (16), min 2
				cfg.Storage.IOWorkers = 8
				cfg.Storage.IOMaxCombineLimit = 128
				cfg.Storage.IOMaxConcurrency = 256
				return cfg
			}(),
		},
		{
			name: "OLTP profile with 8 cores SSD",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        8,
					Profile:         profile.OLTP,
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// factor 0.3, 8 * 0.3 = 2.4 -> ceil = 3
				cfg.Storage.IOWorkers = 3
				cfg.Storage.IOMaxConcurrency = 128
				return cfg
			}(),
		},
		{
			name: "PostgreSQL version 17 should still have defaults but will be zeroed later",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        8 * bytes.GB,
					TotalCPU:        4,
					Profile:         profile.Web,
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 17.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// computeAIO still sets io_workers, computeVersion will zero them
				cfg.Storage.IOWorkers = 2 // 4 * 0.2 = 0.8 -> ceil =1, min2 ->2
				return cfg
			}(),
		},
		{
			name: "Web profile should use factor 0.2",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        10,
					Profile:         profile.Web,
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// 10 * 0.2 = 2
				cfg.Storage.IOWorkers = 2
				return cfg
			}(),
		},
		{
			name: "Mixed profile should use factor 0.25",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        12,
					Profile:         profile.Mixed,
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// 12 * 0.25 = 3
				cfg.Storage.IOWorkers = 3
				return cfg
			}(),
		},
		{
			name: "Unknown profile should use default factor 0.25",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        12,
					Profile:         "unknown_profile",
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// 12 * 0.25 = 3
				cfg.Storage.IOWorkers = 3
				return cfg
			}(),
		},
		{
			name: "SAN disk type should behave like SSD",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        8,
					Profile:         profile.OLTP,
					DiskType:        "SAN",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// OLTP factor 0.3, 8 * 0.3 = 2.4 -> ceil 3
				cfg.Storage.IOWorkers = 3
				cfg.Storage.IOMaxConcurrency = 128
				return cfg
			}(),
		},
		{
			name: "Should limit workers to TotalCPU",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        2,
					Profile:         profile.DW, // factor 0.4
					DiskType:        "HDD",      // +0.1 = 0.5
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// 2 * 0.5 = 1 -> min 2 -> 2. TotalCPU is 2. So workers = 2.
				// Let's try a case where calculation > TotalCPU.
				// Say TotalCPU = 2, Factor = 1.5 (impossible here but hypothetically)
				// Actually with current factors max is 0.5.
				// So workers will always be <= TotalCPU/2? No.
				// If TotalCPU=2, 2*0.5=1 -> min 2.
				// If TotalCPU=4, DW+HDD=0.5 -> 2.
				// Wait, checking code: workers := ceil(TotalCPU * factor).
				// If factor < 1, then workers < TotalCPU always.
				// Except if ceil pushes it up?
				// 4 * 0.9 = 3.6 -> 4.
				// 4 * 1.1 = 4.4 -> 5.
				// Current max factor is 0.4(DW) + 0.1(HDD) = 0.5.
				// So workers will always be <= TotalCPU (since factor < 1).
				// So the check `if workers > in.TotalCPU` might be unreachable with current factors?
				// Let's verify.
				// 1 core: 1 * 0.5 = 0.5 -> ceil 1 -> min 2. workers=2. TotalCPU=1.
				// So workers(2) > TotalCPU(1).
				// Aha! This case hits it.
				cfg.Storage.IOWorkers = 2 // Calculated 2, but TotalCPU is 2. Wait.
				cfg.Storage.IOMaxCombineLimit = 128
				cfg.Storage.IOMaxConcurrency = 256
				return cfg
			}(),
		},
		{
			name: "Should cap workers at TotalCPU when calculated min exceeds it",
			args: args{
				in: &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        4 * bytes.GB,
					TotalCPU:        1,          // Single core
					Profile:         profile.Web, // Factor 0.2
					DiskType:        "SSD",
					MaxConnections:  100,
					PostgresVersion: 18.0,
				},
				cfg: category.NewExportCfg(input.Input{}),
			},
			want: func() *category.ExportCfg {
				cfg := category.NewExportCfg(input.Input{})
				cfg.Storage.IOMethod = "worker"
				// 1 * 0.2 = 0.2 -> ceil 1.
				// min workers check -> sets to 2.
				// max workers check -> if 2 > 1 -> set to 1.
				cfg.Storage.IOWorkers = 1
				return cfg
			}(),
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := computeAIO(tt.args.in, tt.args.cfg)
			if (err != nil) != tt.wantErr {
				t.Errorf("computeAIO() error = %v, wantErr %v", err, tt.wantErr)
				return
			}
			assert.Equal(t, tt.want.Storage.IOMethod, got.Storage.IOMethod, "io_method mismatch")
			assert.Equal(t, tt.want.Storage.IOWorkers, got.Storage.IOWorkers, "io_workers mismatch")
			assert.Equal(t, tt.want.Storage.IOMaxCombineLimit, got.Storage.IOMaxCombineLimit, "io_max_combine_limit mismatch")
			assert.Equal(t, tt.want.Storage.IOMaxConcurrency, got.Storage.IOMaxConcurrency, "io_max_concurrency mismatch")
		})
	}
}