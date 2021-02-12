package category

import "github.com/pgconfig/api/pkg/config"

// CheckpointCfg is the checkpoint related category
type CheckpointCfg struct {
	MinWALSize                 config.Byte `json:"min_wal_size" min_version:"9.5"`
	MaxWALSize                 config.Byte `json:"max_wal_size" min_version:"9.5"` /* pg >= 9.5 */
	CheckpointCompletionTarget float32     `json:"checkpoint_completion_target"`
	WALBuffers                 config.Byte `json:"wal_buffers"`
	CheckpointSegments         int         `json:"checkpoint_segments" max_version:"9.4"` /* pg <= 9.4 */
}

// NewCheckpointCfg creates a new Memory Configuration
//
// For wal_buffers setting automatic by default. check this commit and the comments in the
// function check_wal_buffers on https://github.com/postgres/postgres/commit/2594cf0e8c04406ffff19b1651c5a406d376657c#diff-0cf91b3df8a1bbd72140d10a0b4541b5R4915
func NewCheckpointCfg(in config.Input) *CheckpointCfg {
	return &CheckpointCfg{
		MinWALSize:                 config.Byte(2 * config.GB),
		MaxWALSize:                 config.Byte(3 * config.GB),
		CheckpointCompletionTarget: 0.5,
		WALBuffers:                 -1, // -1 means automatic tuning
		CheckpointSegments:         16,
	}
}

/*
TODO: check the func 'check_wal_buffers' on https://github.com/postgres/postgres/commit/2594cf0e8c04406ffff19b1651c5a406d376657c#diff-0cf91b3df8a1bbd72140d10a0b4541b5R4915
*/
