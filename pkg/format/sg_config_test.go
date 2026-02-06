package format

import (
	"strings"
	"testing"

	"github.com/andreyvit/diff"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestSGConfigFile(t *testing.T) {
	Describe("StackGres config formatting", t, func() {
		It("should format configuration as SGPostgresConfig YAML", func() {
			expected := `
apiVersion: stackgres.io/v1
kind: SGPostgresConfig
metadata:
  name: pgconfig-org-generated
spec:
  postgresVersion: "13"
  postgresql.conf:
    checkpoint_completion_target: "0.5"
    effective_cache_size: 70GB
    effective_io_concurrency: "1"
    maintenance_work_mem: 5GB
    max_connections: "100"
    max_parallel_workers: "2"
    max_parallel_workers_per_gather: "2"
    max_wal_size: 3GB
    max_worker_processes: "8"
    min_wal_size: 2GB
    random_page_cost: "4.0"
    shared_buffers: 23GB
    wal_buffers: "-1"
    work_mem: 965MB
`

			out := SGConfigFile(sliceConfSample, "13")

			actual := strings.TrimSpace(out)
			expectedTrimmed := strings.TrimSpace(expected)

			if actual != expectedTrimmed {
				t.Errorf("Result not as expected:\n%v", diff.LineDiff(expectedTrimmed, actual))
			}
		})
	})
}
