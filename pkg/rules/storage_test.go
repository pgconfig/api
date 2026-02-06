package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeStorage(t *testing.T) {
	Describe("Storage configuration", t, func() {
		It("should configure random_page_cost and io_concurrency based on disk type", func() {
			in := fakeInput()
			in.DiskType = "SSD"
			outSSD, _ := computeStorage(in, category.NewExportCfg(*in))
			in.DiskType = "SAN"
			outSAN, _ := computeStorage(in, category.NewExportCfg(*in))
			in.DiskType = "HDD"
			outHDD, _ := computeStorage(in, category.NewExportCfg(*in))

			Expect(outSSD.Storage.RandomPageCost, ShouldBeLessThanOrEqualTo, 1.1)
			Expect(outSAN.Storage.RandomPageCost, ShouldBeLessThanOrEqualTo, 1.1)

			Expect(outSSD.Storage.EffectiveIOConcurrency, ShouldBeGreaterThanOrEqualTo, 200)
			Expect(outSAN.Storage.EffectiveIOConcurrency, ShouldBeGreaterThanOrEqualTo, 300)

			Expect(outHDD.Storage.EffectiveIOConcurrency, ShouldBeLessThanOrEqualTo, 2)

			Expect(outSSD.Storage.MaintenanceIOConcurrency, ShouldEqual, outSSD.Storage.EffectiveIOConcurrency)
			Expect(outSAN.Storage.MaintenanceIOConcurrency, ShouldEqual, outSAN.Storage.EffectiveIOConcurrency)
			Expect(outHDD.Storage.MaintenanceIOConcurrency, ShouldEqual, outHDD.Storage.EffectiveIOConcurrency)
		})
	})
}

func Test_computeStorageRandomPageCost(t *testing.T) {
	Describe("Random page cost computation", t, func() {
		It("should configure random_page_cost based on profile and disk type", func() {
			tests := []struct {
				name               string
				profile            profile.Profile
				diskType           string
				expectedPageCost   float32
			}{
				{"DW profile with SSD gets higher random_page_cost", profile.DW, "SSD", 1.8},
				{"DW profile with SAN gets higher random_page_cost", profile.DW, "SAN", 1.8},
				{"DW profile with HDD keeps default", profile.DW, "HDD", 4.0},
				{"OLTP profile with SSD gets low random_page_cost", profile.OLTP, "SSD", 1.1},
				{"Web profile with SSD gets low random_page_cost", profile.Web, "SSD", 1.1},
				{"Mixed profile with SSD gets low random_page_cost", profile.Mixed, "SSD", 1.1},
				{"Desktop profile with SSD gets low random_page_cost", profile.Desktop, "SSD", 1.1},
			}

			for _, tt := range tests {
				in := input.Input{
					OS:              "linux",
					Arch:            "amd64",
					Profile:         tt.profile,
					DiskType:        tt.diskType,
					TotalRAM:        100 * bytes.GB,
					MaxConnections:  100,
					TotalCPU:        8,
					PostgresVersion: 16.0,
				}

				out, err := computeStorage(&in, category.NewExportCfg(in))
				Expect(err, ShouldBeNil)
				Expect(out.Storage.RandomPageCost, ShouldEqual, tt.expectedPageCost)
			}
		})
	})
}
