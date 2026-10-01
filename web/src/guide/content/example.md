# Full example

The example below its used by the `UI`:

```bash
$ curl 'https://api.pgconfig.org/v1/tuning/get-config?environment_name=WEB&format=alter_system&include_pgbadger=true&log_format=stderr&max_connections=100&pg_version=9.6&total_ram=2GB'
```

## How the values are calculated?

Default values are set in the [`pkg/category` package](https://github.com/pgconfig/api/tree/main/pkg/category). Take by example the [Checkpoint Configuration category](https://github.com/pgconfig/api/blob/main/pkg/category/checkpoint.go#L18-L26):

```go
// ...
	return &CheckpointCfg{
		MinWALSize:                 config.Byte(2 * config.GB),
		MaxWALSize:                 config.Byte(3 * config.GB),
		CheckpointCompletionTarget: 0.5,
		WALBuffers:                 -1, // -1 means automatic tuning
		CheckpointSegments:         16,
	}
```

Once the default values are set, they are computed based in the input, set in the [`pkg/rules` package](https://github.com/pgconfig/api/tree/main/pkg/rules). Take by example [the storage rules](https://github.com/pgconfig/api/blob/main/pkg/rules/storage.go#L8-L24):

```go
// ...
func computeStorage(in *config.Input, cfg *category.ExportCfg) (*category.ExportCfg, error) {

	switch in.DiskType {
	case "SSD":
		cfg.Storage.EffectiveIOConcurrency = 200
	case "SAN":
		cfg.Storage.EffectiveIOConcurrency = 300
	default:
		cfg.Storage.EffectiveIOConcurrency = 2
	}

	if in.DiskType != "HDD" {
		cfg.Storage.RandomPageCost = 1.1
	}

	return cfg, nil
}
```

The [rules are computed in the following order](https://github.com/pgconfig/api/blob/main/pkg/rules/compute.go#L12-L22):

1. Arch
1. OS
1. Profile
1. Storage
1. Postgres Version
