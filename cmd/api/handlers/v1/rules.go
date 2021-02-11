package v1

import (
	"strconv"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/docs"
)

const defaultPgVersion = "13"

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
