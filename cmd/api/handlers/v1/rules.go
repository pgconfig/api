package v1

import (
	"io/ioutil"
	"log"
	"strconv"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/docs"
	"gopkg.in/yaml.v2"
)

const defaultPgVersion = "13"

var (
	allCategories rulesFile
	pgDocs        docs.DocFile
)

func init() {
	fileData, err := ioutil.ReadFile("./../../rules.yml")
	if err != nil {
		log.Fatalf("could not open rules config file: %v", err)
	}

	err = yaml.Unmarshal(fileData, &allCategories)
	if err != nil {
		log.Fatalf("could not parse rules config file: %v", err)
	}
	docFile, err := ioutil.ReadFile("./../../pg-docs.yml")
	if err != nil {
		log.Fatalf("could not open pg docs file: %v", err)
	}

	err = yaml.Unmarshal(docFile, &pgDocs)
	if err != nil {
		log.Fatalf("could not parse pg docs file: %v", err)
	}
}

// GetRules is a function to list all categories and parameters rules
// @Summary Get a list of rules
// @Description list of categories and parameters rules (used to compute get-config)
// @Accept json
// @Produce json
// @Param pg_version query string false "PostgreSQL Version" default(13)
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-rules [get]
func GetRules(c *fiber.Ctx) error {
	ver, err := strconv.ParseFloat(c.Query("pg_version", defaultPgVersion), 32)

	if err != nil {
		return err
	}
	pgVersion := docs.FormatVer(float32(ver))

	return c.JSON(v1Reponse(c, addDocTORules(pgVersion, true)))
}

func addDocTORules(pgVersion string, showDoc bool) []outputCategory {
	var output = make([]outputCategory, len(allCategories.Categories))
	copy(output, allCategories.Categories)

	for c := 0; c < len(output); c++ {
		for p := 0; p < len(output[c].Parameters); p++ {
			if !showDoc {
				output[c].Parameters[p].Documentation = nil
				continue
			}

			paramDoc := pgDocs.Documentation[pgVersion][output[c].Parameters[p].Name]
			output[c].Parameters[p].Documentation = &paramDoc
			output[c].Parameters[p].Documentation.Abstract = output[c].Parameters[p].Notes.Abstract
			output[c].Parameters[p].Documentation.BlogRecomendations = output[c].Parameters[p].Notes.Recomendations
		}
	}

	return output
}

type rulesFile struct {
	Categories []outputCategory `json:"categories"`
}

type notes struct {
	Abstract       string            `json:"abstract"`
	Recomendations map[string]string `json:"recomendations,omitempty"`
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
