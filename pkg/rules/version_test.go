package rules

import (
	"testing"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/input/bytes"
	. "github.com/pgconfig/api/pkg/tests"
)

func Test_computeVersion(t *testing.T) {
	Describe("PostgreSQL version-specific settings", t, func() {
		Context("versions older than 9.5", func() {
			It("should remove min_wal_size and max_wal_size", func() {
				in := fakeInput()
				in.PostgresVersion = 9.4

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Checkpoint.MinWALSize, ShouldEqual, 0)
				Expect(out.Checkpoint.MaxWALSize, ShouldEqual, 0)
			})

			It("should limit shared_buffers up to 8gb", func() {
				in := fakeInput()
				in.PostgresVersion = 9.5
				in.TotalRAM = 1 * bytes.TB

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Memory.SharedBuffers, ShouldBeLessThanOrEqualTo, 8*bytes.GB)
			})
		})

		Context("versions 9.5 and newer", func() {
			It("should remove checkpoint_segments", func() {
				in := fakeInput()
				in.PostgresVersion = 9.5

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Checkpoint.CheckpointSegments, ShouldEqual, 0)
			})
		})

		Context("versions older than 9.6", func() {
			It("should remove max_parallel_workers_per_gather", func() {
				in := fakeInput()
				in.PostgresVersion = 9.4

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Worker.MaxParallelWorkerPerGather, ShouldEqual, 0)
			})

			It("should remove the workers category in versions older than 9.3", func() {
				in := fakeInput()
				in.PostgresVersion = 9.3

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Worker, ShouldBeNil)
			})
		})

		Context("versions older than 10", func() {
			It("should remove max_parallel_workers", func() {
				in := fakeInput()
				in.PostgresVersion = 9.5

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Worker.MaxParallelWorkers, ShouldEqual, 0)
			})
		})

		Context("versions older than 13", func() {
			It("should zero maintenance_io_concurrency", func() {
				in := fakeInput()
				in.PostgresVersion = 12.0

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Storage.MaintenanceIOConcurrency, ShouldEqual, 0)
			})
		})

		Context("versions 13 and newer", func() {
			It("should keep maintenance_io_concurrency", func() {
				in := fakeInput()
				in.PostgresVersion = 13.0

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Storage.MaintenanceIOConcurrency, ShouldNotEqual, 0)
			})
		})

		Context("versions older than 18", func() {
			It("should zero all AIO-related parameters", func() {
				in := fakeInput()
				in.PostgresVersion = 17.0

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Storage.IOMethod, ShouldEqual, "")
				Expect(out.Storage.IOWorkers, ShouldEqual, 0)
				Expect(out.Storage.IOMaxCombineLimit, ShouldEqual, 0)
				Expect(out.Storage.IOMaxConcurrency, ShouldEqual, 0)
				Expect(out.Storage.FileCopyMethod, ShouldEqual, "")
			})
		})

		Context("versions 18 and newer", func() {
			It("should keep AIO-related parameters", func() {
				in := fakeInput()
				in.PostgresVersion = 18.0

				out, _ := computeVersion(in, category.NewExportCfg(*in))

				Expect(out.Storage.IOMethod, ShouldNotEqual, "")
				Expect(out.Storage.IOWorkers, ShouldNotEqual, 0)
				Expect(out.Storage.IOMaxCombineLimit, ShouldNotEqual, 0)
				Expect(out.Storage.IOMaxConcurrency, ShouldNotEqual, 0)
				Expect(out.Storage.FileCopyMethod, ShouldNotEqual, "")
			})
		})
	})
}
