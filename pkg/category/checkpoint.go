package category

import "github.com/pgconfig/api/pkg/config"

// CheckpointCfg is the checkpoint related category
type CheckpointCfg struct {
	MinWALSize                 int     `json:"min_wal_size"`
	MaxWALSize                 int     `json:"max_wal_size"` /* pg >= 9.5 */
	CheckpointCompletionTarget float32 `json:"checkpoint_completion_target"`
	WALBuffers                 int     `json:"wal_buffers"`
	CheckpointSegments         int     `json:"checkpoint_segments"` /* pg <= 9.4 */
}

// NewCheckpointCfg creates a new Memory Configuration
func NewCheckpointCfg(in config.Input) *CheckpointCfg {
	return &CheckpointCfg{
		MinWALSize:                 2 * config.GB,
		MaxWALSize:                 3 * config.GB,
		CheckpointCompletionTarget: 0.5,
		WALBuffers:                 int(float32((in.TotalRAM / 16)) * 0.03),
		CheckpointSegments:         16,
	}
}

/*
TODO: check the func 'check_wal_buffers' on https://github.com/postgres/postgres/commit/2594cf0e8c04406ffff19b1651c5a406d376657c#diff-0cf91b3df8a1bbd72140d10a0b4541b5R4915
*/
