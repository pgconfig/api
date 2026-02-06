package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeOS(t *testing.T) {
	Describe("Operating system handling", t, func() {
		It("should return error on non-supported operating systems", func() {
			_, err := computeOS(&input.Input{OS: "xpto-wrong-os"}, &category.ExportCfg{})
			Expect(err, ShouldResemble, errors.ErrorInvalidOS)
		})

		It("should ignore case for all operating systems supported", func() {
			in := fakeInput()
			in.OS = "lINUx"
			in.TotalRAM = 120 * bytes.GB

			_, err := computeOS(in, category.NewExportCfg(*in))
			Expect(err, ShouldBeNil)
		})

		Context("Windows specific limits", func() {
			It("should limit shared_buffers to 512MB until pg 10", func() {
				in := fakeInput()
				in.OS = "windows"
				in.PostgresVersion = 9.6

				out, err := computeOS(in, category.NewExportCfg(*in))
				Expect(err, ShouldBeNil)
				Expect(out.Memory.SharedBuffers, ShouldEqual, 512*bytes.MB)
			})

			It("should limit effective_io_concurrency to 0 on platforms that lack posix_fadvise()", func() {
				in := fakeInput()
				in.OS = Windows
				in.PostgresVersion = 12.0

				out, err := computeOS(in, category.NewExportCfg(*in))
				Expect(err, ShouldBeNil)
				Expect(out.Storage.EffectiveIOConcurrency, ShouldEqual, 0)
			})

			It("should not limit shared_buffers on versions >= pg 11", func() {
				in := fakeInput()
				in.PostgresVersion = 14.0
				in.TotalRAM = 120 * bytes.GB

				out, err := computeOS(in, category.NewExportCfg(*in))
				Expect(err, ShouldBeNil)
				Expect(out.Memory.SharedBuffers, ShouldBeGreaterThan, 25*bytes.GB)
			})

			It("should limit work_mem to ~2GB on Windows (issue #5)", func() {
				in := fakeInput()
				in.OS = Windows
				in.TotalRAM = 256 * bytes.GB
				in.MaxConnections = 10
				in.PostgresVersion = 16.0

				cfg := category.NewExportCfg(*in)
				cfg.Memory.WorkMem = 5 * bytes.GB

				out, err := computeOS(in, cfg)
				Expect(err, ShouldBeNil)
				Expect(out.Memory.WorkMem, ShouldEqual, WindowsMaxWorkMem)
				Expect(out.Memory.WorkMem, ShouldBeLessThan, 2*bytes.GB)
			})

			It("should limit maintenance_work_mem to ~2GB on Windows (issue #5)", func() {
				in := fakeInput()
				in.OS = Windows
				in.TotalRAM = 256 * bytes.GB
				in.PostgresVersion = 16.0

				cfg := category.NewExportCfg(*in)
				cfg.Memory.MaintenanceWorkMem = 10 * bytes.GB

				out, err := computeOS(in, cfg)
				Expect(err, ShouldBeNil)
				Expect(out.Memory.MaintenanceWorkMem, ShouldEqual, WindowsMaxWorkMem)
				Expect(out.Memory.MaintenanceWorkMem, ShouldBeLessThan, 2*bytes.GB)
			})

			It("should not limit work_mem on non-Windows platforms", func() {
				in := fakeInput()
				in.OS = Linux
				in.TotalRAM = 256 * bytes.GB
				in.MaxConnections = 10
				in.PostgresVersion = 16.0

				cfg := category.NewExportCfg(*in)
				cfg.Memory.WorkMem = 5 * bytes.GB

				out, err := computeOS(in, cfg)
				Expect(err, ShouldBeNil)
				Expect(out.Memory.WorkMem, ShouldEqual, 5*bytes.GB)
			})

			It("should not limit work_mem on Windows with PostgreSQL 18+", func() {
				in := fakeInput()
				in.OS = Windows
				in.TotalRAM = 256 * bytes.GB
				in.MaxConnections = 10
				in.PostgresVersion = 18.0

				cfg := category.NewExportCfg(*in)
				cfg.Memory.WorkMem = 5 * bytes.GB

				out, err := computeOS(in, cfg)
				Expect(err, ShouldBeNil)
				Expect(out.Memory.WorkMem, ShouldEqual, 5*bytes.GB)
			})

			It("should not limit maintenance_work_mem on Windows with PostgreSQL 18+", func() {
				in := fakeInput()
				in.OS = Windows
				in.TotalRAM = 256 * bytes.GB
				in.PostgresVersion = 18.0

				cfg := category.NewExportCfg(*in)
				cfg.Memory.MaintenanceWorkMem = 10 * bytes.GB

				out, err := computeOS(in, cfg)
				Expect(err, ShouldBeNil)
				Expect(out.Memory.MaintenanceWorkMem, ShouldEqual, 10*bytes.GB)
			})
		})
	})
}
