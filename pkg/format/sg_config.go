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

// apiVersion: stackgres.io/v1
// kind: SGPostgresConfig
// metadata:
//   name: postgresconf
// spec:
//   postgresVersion: "11"
//   postgresql.conf:
//     password_encryption: 'scram-sha-256'
//     random_page_cost: '1.5'
//     shared_buffers: '256MB'
//     wal_compression: 'on'

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
			out.Spec.Config[param.Name] = param.Value
		}
	}

	d, _ := yaml.Marshal(&out)

	return string(d)
}
