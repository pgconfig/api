package format

import (
	"github.com/pgconfig/api/pkg/category"
	"gopkg.in/yaml.v2"
)

type sgPostgresConfig struct {
	Apiversion string   `yaml:"apiVersion"`
	Kind       string   `yaml:"kind"`
	Metadata   metadata `yaml:"metadata"`
	Spec       spec     `yaml:"spec"`
}
type metadata struct {
	Name string `yaml:"name"`
}

type spec struct {
	Postgresversion string            `yaml:"postgresVersion"`
	Config          map[string]string `yaml:"postgresql.conf"`
}

// SGConfigFile exports the config file in the CRD SGPostgresConfig
// used by the Stackgres.io operator.
func SGConfigFile(report []category.SliceOutput, pgVersion string) string {

	out := sgPostgresConfig{
		Apiversion: "stackgres.io/v1",
		Kind:       "SGPostgresConfig",
		Metadata: metadata{
			Name: "pgconfig-org-generated",
		},
		Spec: spec{
			Postgresversion: pgVersion,
			Config:          make(map[string]string),
		},
	}

	for _, cat := range report {
		for _, param := range cat.Parameters {
			if sgBlockedParam(param.Name) {
				continue
			}

			out.Spec.Config[param.Name] = param.Value
		}
	}

	d, _ := yaml.Marshal(&out)

	return string(d)
}

// from https://github.com/ongres/stackgres/blob/main/stackgres-k8s/src/operator/src/main/resources/postgresql-blocklist.properties
var blockedParamsForSG = []string{
	"listen_addresses",
	"port",
	"hot_standby",
	"fsync",
	"logging_collector",
	"log_destination",
	"log_directory",
	"log_filename",
	"log_rotation_age",
	"log_rotation_size",
	"log_truncate_on_rotation",
	"wal_level",
	"track_commit_timestamp",
	"wal_log_hints",
	"archive_mode",
	"archive_command",
	"lc_messages",
	"wal_compression",
	"dynamic_library_path",
}

// sgBlockedParam checks if the param is in the StackGres
// Blocked list and removes it from the output
func sgBlockedParam(paramName string) bool {
	for i := 0; i < len(blockedParamsForSG); i++ {
		if blockedParamsForSG[i] == paramName {
			return true
		}
	}

	return false
}
