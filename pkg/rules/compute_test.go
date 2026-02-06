package rules

import (
	"testing"

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
