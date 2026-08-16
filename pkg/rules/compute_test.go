package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/format"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestCompute(t *testing.T) {
	t.SkipNow()
}

func TestComputeWalSizes(t *testing.T) {
	Describe("WAL size computation", t, func() {
		It("should configure WAL sizes based on profile", func() {
			tests := []struct {
				name           string
				profile        profile.Profile
				totalRAM       bytes.Byte
				expectedMinWAL bytes.Byte
				expectedMaxWAL bytes.Byte
			}{
				{"DW profile gets large WAL sizes", profile.DW, 100 * bytes.GB, 4 * bytes.GB, 16 * bytes.GB},
				{"OLTP profile gets medium-large WAL sizes", profile.OLTP, 100 * bytes.GB, 2 * bytes.GB, 8 * bytes.GB},
				{"Web profile gets moderate WAL sizes", profile.Web, 100 * bytes.GB, 1 * bytes.GB, 4 * bytes.GB},
				{"Mixed profile gets balanced WAL sizes", profile.Mixed, 100 * bytes.GB, 2 * bytes.GB, 6 * bytes.GB},
				{"Desktop profile gets small WAL sizes", profile.Desktop, 16 * bytes.GB, 512 * bytes.MB, 2 * bytes.GB},
			}

			for _, tt := range tests {
				in := input.Input{
					OS:              "linux",
					Arch:            "amd64",
					Profile:         tt.profile,
					TotalRAM:        tt.totalRAM,
					MaxConnections:  100,
					DiskType:        "ssd",
					TotalCPU:        8,
					PostgresVersion: 16.0,
				}

				out, err := Compute(in)
				Expect(err, ShouldBeNil)
				Expect(out.Checkpoint.MinWALSize, ShouldEqual, tt.expectedMinWAL)
				Expect(out.Checkpoint.MaxWALSize, ShouldEqual, tt.expectedMaxWAL)
			}
		})
	})
}

func TestComputeWalBuffers(t *testing.T) {
	Describe("WAL buffers computation", t, func() {
		It("should configure wal_buffers based on profile and RAM", func() {
			tests := []struct {
				name               string
				profile            profile.Profile
				totalRAM           bytes.Byte
				expectedWalBuffers bytes.Byte
			}{
				{"DW profile always gets 64MB", profile.DW, 100 * bytes.GB, 64 * bytes.MB},
				{"DW profile with small RAM still gets 64MB", profile.DW, 8 * bytes.GB, 64 * bytes.MB},
				{"OLTP with large shared_buffers gets 32MB", profile.OLTP, 40 * bytes.GB, 32 * bytes.MB},
				{"OLTP with small shared_buffers uses auto-tune", profile.OLTP, 16 * bytes.GB, -1},
				{"Web profile uses auto-tune", profile.Web, 100 * bytes.GB, -1},
				{"Mixed profile uses auto-tune", profile.Mixed, 100 * bytes.GB, -1},
				{"Desktop profile uses auto-tune", profile.Desktop, 16 * bytes.GB, -1},
			}

			for _, tt := range tests {
				in := input.Input{
					OS:              "linux",
					Arch:            "amd64",
					Profile:         tt.profile,
					TotalRAM:        tt.totalRAM,
					MaxConnections:  100,
					DiskType:        "ssd",
					TotalCPU:        8,
					PostgresVersion: 16.0,
				}

				out, err := Compute(in)
				Expect(err, ShouldBeNil)
				Expect(out.Checkpoint.WALBuffers, ShouldEqual, tt.expectedWalBuffers)
			}
		})
	})
}

func TestComputeIOWorkersClampAcrossFormats(t *testing.T) {
	Describe("io_workers clamp in output formats", t, func() {
		It("should emit the clamped value in json, alter_system, and conf", func() {
			in := input.Input{
				OS:              "linux",
				Arch:            "amd64",
				Profile:         profile.DW,
				TotalRAM:        540 * bytes.GB,
				MaxConnections:  25,
				DiskType:        "SSD",
				TotalCPU:        96,
				PostgresVersion: 18.0,
			}

			out, err := Compute(in)
			Expect(err, ShouldBeNil)
			Expect(out.Storage.IOWorkers, ShouldEqual, 32)

			report := out.ToSlice(18.0, false, "")

			Expect(format.ExportConf(format.JSON, report, 18.0, nil),
				ShouldContainSubstring, `"name": "io_workers"`)
			Expect(format.ExportConf(format.JSON, report, 18.0, nil),
				ShouldContainSubstring, `"config_value": "32"`)
			Expect(format.ExportConf(format.AlterSystemFormat, report, 18.0, nil),
				ShouldContainSubstring, "ALTER SYSTEM SET io_workers TO '32';")
			Expect(format.ExportConf(format.Config, report, 18.0, nil),
				ShouldContainSubstring, "io_workers = 32")
		})
	})
}
