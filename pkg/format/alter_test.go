package format

import (
	"strings"
	"testing"

	"github.com/andreyvit/diff"
)

func TestAlterSystem(t *testing.T) {
	sample := `
-- Memory Configuration
ALTER SYSTEM SET shared_buffers TO '23GB';
ALTER SYSTEM SET effective_cache_size TO '70GB';
ALTER SYSTEM SET work_mem TO '965MB';
ALTER SYSTEM SET maintenance_work_mem TO '5GB';

-- Checkpoint Related Configuration
ALTER SYSTEM SET min_wal_size TO '2GB';
ALTER SYSTEM SET max_wal_size TO '3GB';
ALTER SYSTEM SET checkpoint_completion_target TO '0.5';
ALTER SYSTEM SET wal_buffers TO '-1';

-- Network Related Configuration
ALTER SYSTEM SET listen_addresses TO '*';
ALTER SYSTEM SET max_connections TO '100';

-- Storage Configuration
ALTER SYSTEM SET random_page_cost TO '4.0';
ALTER SYSTEM SET effective_io_concurrency TO '1';

-- Worker Processes Configuration
ALTER SYSTEM SET max_worker_processes TO '8';
ALTER SYSTEM SET max_parallel_workers_per_gather TO '2';
ALTER SYSTEM SET max_parallel_workers TO '2';
`

	out := AlterSystem(sliceConfSample)

	if a, e := strings.TrimSpace(sample), strings.TrimSpace(out); a != e {
		t.Errorf("Result not as expected:\n%v", diff.LineDiff(e, a))
	}
}
