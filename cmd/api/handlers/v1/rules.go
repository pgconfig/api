package v1

import (
	"io/ioutil"
	"log"

	"github.com/gofiber/fiber/v2"
	"gopkg.in/yaml.v2"
)

var allCategories rulesFile

func init() {
	fileData, err := ioutil.ReadFile("./../../rules.yml")
	if err != nil {
		log.Fatalf("could not open rules config file: %v", err)
	}

	err = yaml.Unmarshal(fileData, &allCategories)
	if err != nil {
		log.Fatalf("could not parse rules config file: %v", err)
	}
}

// GetRules is a function to list all categories and parameters rules
// @Summary Get a list of rules
// @Description list of categories and parameters rules (used to compute get-config)
// @Accept json
// @Produce json
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-rules [get]
func GetRules(c *fiber.Ctx) error {
	return c.JSON(v1Reponse(c, allCategories.Categories))
}

type rulesFile struct {
	Categories []category `json:"categories"`
}

type documentation struct {
	Abstract       string            `json:"abstract"`
	Recomendations map[string]string `json:"recomendations,omitempty"`
	Type           string            `json:"type"`
	URL            string            `json:"url"`
}
type parameter struct {
	Documentation documentation `json:"documentation"`
	Format        string        `json:"format"`
	Formula       string        `json:"formula"`
	Name          string        `json:"name"`
}
type category struct {
	Name        string      `json:"category" yaml:"name"`
	Description string      `json:"description"`
	Parameters  []parameter `json:"parameters"`
}
