package v1

import (
	"io/ioutil"
	"log"

	"github.com/pgconfig/api/pkg/docs"
	"gopkg.in/yaml.v2"
)

type rulesFileContent struct {
	Categories map[string]map[string]parameter `json:"categories"`
}

type parameter struct {
	Abstract       string            `json:"abstract"`
	Recomendations map[string]string `json:"recomendations,omitempty"`
	Formula        string            `json:"formula,omitempty"`
}

var (
	allRules rulesFileContent
	pgDocs   docs.DocFile
)

// Prepare loads the necessary files to the api server
func Prepare(rulesFile, docsFile string) {
	fileData, err := ioutil.ReadFile(rulesFile)
	if err != nil {
		log.Fatalf("could not open rules config file: %v", err)
	}

	err = yaml.Unmarshal(fileData, &allRules)
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
