package category

import (
	"testing"

	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestNewWorkerCfg(t *testing.T) {
	Describe("Worker configuration", t, func() {
		It("should configure workers based on profile and CPU count", func() {
			tests := []struct {
				name                              string
				profile                           profile.Profile
				totalCPU                          int
				expectedMaxWorkerProcesses        int
				expectedMaxParallelWorkers        int
				expectedMaxParallelWorkerPerGather int
			}{
				{"Desktop with 4 cores", profile.Desktop, 4, 8, 8, 2},
				{"Web with 8 cores", profile.Web, 8, 8, 8, 2},
				{"OLTP with 16 cores", profile.OLTP, 16, 16, 16, 2},
				{"Mixed with 16 cores", profile.Mixed, 16, 16, 16, 2},
				{"DW with 8 cores", profile.DW, 8, 8, 8, 4},
				{"DW with 16 cores", profile.DW, 16, 16, 16, 8},
				{"DW with 32 cores", profile.DW, 32, 32, 32, 16},
				{"DW with 2 cores ensures minimum", profile.DW, 2, 8, 8, 2},
				{"Large system with 64 cores", profile.OLTP, 64, 64, 64, 2},
			}

			for _, tt := range tests {
				in := input.Input{
					OS:              "linux",
					Arch:            "amd64",
					Profile:         tt.profile,
					TotalCPU:        tt.totalCPU,
					TotalRAM:        16 * bytes.GB,
					MaxConnections:  100,
					DiskType:        "SSD",
					PostgresVersion: 16.0,
				}

				cfg := NewWorkerCfg(in)

				Expect(cfg.MaxWorkerProcesses, ShouldEqual, tt.expectedMaxWorkerProcesses)
				Expect(cfg.MaxParallelWorkers, ShouldEqual, tt.expectedMaxParallelWorkers)
				Expect(cfg.MaxParallelWorkerPerGather, ShouldEqual, tt.expectedMaxParallelWorkerPerGather)
			}
		})
	})
}
