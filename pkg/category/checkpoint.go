package category

import (
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
)

// CheckpointCfg is the checkpoint related category
type CheckpointCfg struct {
	MinWALSize                 bytes.Byte `json:"min_wal_size" min_version:"9.5"`
	MaxWALSize                 bytes.Byte `json:"max_wal_size" min_version:"9.5"` /* pg >= 9.5 */
	CheckpointCompletionTarget float32    `json:"checkpoint_completion_target"`
	WALBuffers                 bytes.Byte `json:"wal_buffers"`
	CheckpointSegments         int        `json:"checkpoint_segments" max_version:"9.4"` /* pg <= 9.4 */
}

// NewCheckpointCfg creates a new Memory Configuration
//
// For wal_buffers setting automatic by default. check this commit and the comments in the
// function check_wal_buffers on https://github.com/postgres/postgres/commit/2594cf0e8c04406ffff19b1651c5a406d376657c#diff-0cf91b3df8a1bbd72140d10a0b4541b5R4915
func NewCheckpointCfg(in input.Input) *CheckpointCfg {
	// Calculate shared_buffers to determine wal_buffers tuning
	// Same calculation as in memory.go: totalRAM * MaxMemoryProfilePercent * SharedBufferPerc (25%)
	maxMemoryProfilePercent := map[profile.Profile]float32{
		profile.Web:     1,
		profile.OLTP:    1,
		profile.DW:      1,
		profile.Mixed:   0.5,
		profile.Desktop: 0.2,
	}
	totalRAM := float32(in.TotalRAM) * maxMemoryProfilePercent[in.Profile]
	sharedBuffers := bytes.Byte(totalRAM * 0.25) // SharedBufferPerc = 0.25

	// DW (Data Warehouse) workloads benefit from larger wal_buffers
	// for write-heavy operations
	// OLTP with large shared_buffers (>8GB) indicates high concurrent writes
	walBuffers := bytes.Byte(-1) // -1 means automatic tuning

	if in.Profile == profile.DW {
		walBuffers = 64 * bytes.MB
	} else if in.Profile == profile.OLTP && sharedBuffers > 8*bytes.GB {
		walBuffers = 32 * bytes.MB
	}

	// WAL size tuning per profile
	// Recommended to hold ~1 hour of WAL for most systems
	minWALSize := bytes.Byte(2 * bytes.GB)
	maxWALSize := bytes.Byte(3 * bytes.GB)

	switch in.Profile {
	case profile.DW:
		// Data Warehouse: write-heavy with batch jobs
		minWALSize = 4 * bytes.GB
		maxWALSize = 16 * bytes.GB
	case profile.OLTP:
		// OLTP: frequent transactions
		minWALSize = 2 * bytes.GB
		maxWALSize = 8 * bytes.GB
	case profile.Web:
		// Web: moderate writes
		minWALSize = 1 * bytes.GB
		maxWALSize = 4 * bytes.GB
	case profile.Mixed:
		// Mixed: balanced workload
		minWALSize = 2 * bytes.GB
		maxWALSize = 6 * bytes.GB
	case profile.Desktop:
		// Desktop: low activity
		minWALSize = 512 * bytes.MB
		maxWALSize = 2 * bytes.GB
	}

	return &CheckpointCfg{
		MinWALSize:                 minWALSize,
		MaxWALSize:                 maxWALSize,
		CheckpointCompletionTarget: 0.9,
		WALBuffers:                 walBuffers,
		CheckpointSegments:         16,
	}
}

/*
TODO: check the func 'check_wal_buffers' on https://github.com/postgres/postgres/commit/2594cf0e8c04406ffff19b1651c5a406d376657c#diff-0cf91b3df8a1bbd72140d10a0b4541b5R4915
*/
