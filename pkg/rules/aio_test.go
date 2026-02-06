package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeAIO(t *testing.T) {
	Describe("AIO (Asynchronous I/O) computation", t, func() {
		It("should configure io_workers based on profile and CPU count", func() {
			tests := []struct {
				name                 string
				profile              profile.Profile
				totalCPU             int
				diskType             string
				pgVersion            float64
				expectedIOWorkers    int
				expectedIOMethod     string
				expectedCombineLimit int
				expectedConcurrency  int
			}{
				{"Desktop with 4 cores", profile.Desktop, 4, "SSD", 18.0, 2, "worker", 16, 64},
				{"DW profile with 16 cores HDD", profile.DW, 16, "HDD", 18.0, 8, "worker", 128, 256},
				{"OLTP profile with 8 cores SSD", profile.OLTP, 8, "SSD", 18.0, 3, "worker", 16, 128},
				{"PostgreSQL version 17 still gets values", profile.Web, 4, "SSD", 17.0, 2, "worker", 16, 64},
				{"Web profile uses factor 0.2", profile.Web, 10, "SSD", 18.0, 2, "worker", 16, 64},
				{"Mixed profile uses factor 0.25", profile.Mixed, 12, "SSD", 18.0, 3, "worker", 16, 64},
				{"Unknown profile uses default factor 0.25", "unknown_profile", 12, "SSD", 18.0, 3, "worker", 16, 64},
				{"SAN disk type behaves like SSD", profile.OLTP, 8, "SAN", 18.0, 3, "worker", 16, 128},
				{"Should limit workers to TotalCPU", profile.DW, 2, "HDD", 18.0, 2, "worker", 128, 256},
				{"Should cap workers at TotalCPU when min exceeds it", profile.Web, 1, "SSD", 18.0, 1, "worker", 16, 64},
			}

			for _, tt := range tests {
				in := &input.Input{
					OS:              "linux",
					Arch:            "amd64",
					TotalRAM:        16 * bytes.GB,
					TotalCPU:        tt.totalCPU,
					Profile:         tt.profile,
					DiskType:        tt.diskType,
					MaxConnections:  100,
					PostgresVersion: float32(tt.pgVersion),
				}
				cfg := category.NewExportCfg(input.Input{})

				got, err := computeAIO(in, cfg)

				Expect(err, ShouldBeNil)
				Expect(got.Storage.IOMethod, ShouldEqual, tt.expectedIOMethod)
				Expect(got.Storage.IOWorkers, ShouldEqual, tt.expectedIOWorkers)
				Expect(got.Storage.IOMaxCombineLimit, ShouldEqual, tt.expectedCombineLimit)
				Expect(got.Storage.IOMaxConcurrency, ShouldEqual, tt.expectedConcurrency)
			}
		})
	})
}
