package v1

import (
	"io/ioutil"
	"log"

	"github.com/pgconfig/api/pkg/docs"
	"gopkg.in/yaml.v2"
)

type rulesFileContent struct {
	Categories []outputCategory `json:"categories"`
}

type notes struct {
	Abstract       string            `json:"abstract"`
	Recomendations map[string]string `json:"recomendations,omitempty"`
	Value          string            `json:"config_value,omitempty"`
	Comment        string            `json:"comment,omitempty"`
}
type parameter struct {
	Notes         notes          `yaml:"notes" json:"-"`
	Documentation *docs.ParamDoc `json:"documentation,omitempty"`
	Format        string         `json:"format"`
	Formula       string         `json:"formula"`
	Name          string         `json:"name"`
	Value         string         `json:"config_value,omitempty"`
}
type outputCategory struct {
	Name        string       `json:"category" yaml:"name"`
	Description string       `json:"description"`
	Parameters  []*parameter `json:"parameters"`
}

var (
	allCategories rulesFileContent
	pgDocs        docs.DocFile
)

// Prepare loads the necessary files to the api server
func Prepare(rulesFile, docsFile string) {
	fileData, err := ioutil.ReadFile(rulesFile)
	if err != nil {
		log.Fatalf("could not open rules config file: %v", err)
	}

	err = yaml.Unmarshal(fileData, &allCategories)
	if err != nil {
		log.Fatalf("could not parse rules config file: %v", err)
	}
	docFile, err := ioutil.ReadFile(docsFile)
	if err != nil {
		log.Fatalf("could not open pg docs file: %v", err)
	}

	err = yaml.Unmarshal(docFile, &pgDocs)
	if err != nil {
		log.Fatalf("could not parse pg docs file: %v", err)
	}
}
