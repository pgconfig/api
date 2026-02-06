package rules

import (
	"strings"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
)

const (
	Windows = "windows"
	Linux   = "linux"
	Unix    = "unix"
	Darwin  = "darwin"

	// WindowsMaxWorkMem is the maximum work_mem/maintenance_work_mem on Windows for PostgreSQL <= 17
	// PostgreSQL used MAX_KILOBYTES = INT_MAX/1024 when SIZEOF_LONG <= 4
	// Windows LLP64 model has sizeof(long)==4 even on 64-bit systems
	// This resulted in max value of 2097151 kB (~2GB) on Windows
	// Fixed in PostgreSQL 18 by removing SIZEOF_LONG check from MAX_KILOBYTES
	// Mailing list: https://www.postgresql.org/message-id/flat/1a01f0-66ec2d80-3b-68487680@27595217
	// Related: https://github.com/pgvector/pgvector/issues/667
	WindowsMaxWorkMem = 2097151 * bytes.KB
)

// ValidOS validates the Operating System
func ValidOS(os string) error {
	switch strings.ToLower(os) {
	case Windows:
	case Linux:
	case Unix, Darwin:
	default:
		return errors.ErrorInvalidOS
	}

	return nil
}

func computeOS(in *input.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	var err error

	if err = ValidOS(in.OS); err != nil {
		return nil, err
	}

	if cfg.Memory.SharedBuffers > 512*bytes.MB && in.PostgresVersion <= 9.6 {
		cfg.Memory.SharedBuffers = 512 * bytes.MB
	}

	if in.OS == "windows" {
		cfg.Storage.EffectiveIOConcurrency = 0

		// Windows had 2GB limitation for work_mem and maintenance_work_mem on PG <= 17
		// Fixed in PostgreSQL 18: https://www.postgresql.org/message-id/flat/1a01f0-66ec2d80-3b-68487680@27595217
		if in.PostgresVersion < 18.0 {
			if cfg.Memory.WorkMem > WindowsMaxWorkMem {
				cfg.Memory.WorkMem = WindowsMaxWorkMem
			}
			if cfg.Memory.MaintenanceWorkMem > WindowsMaxWorkMem {
				cfg.Memory.MaintenanceWorkMem = WindowsMaxWorkMem
			}
		}
	}

	return cfg, nil
}
