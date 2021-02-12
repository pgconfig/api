package v1

import (
	"github.com/gofiber/fiber/v2"
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
	return nil
	// ver, err := strconv.ParseFloat(c.Query("pg_version", defaultPgVersion), 32)

	// if err != nil {
	// 	return err
	// }
	// pgVersion := docs.FormatVer(float32(ver))

	// return c.JSON(v1Reponse(c, addDocTORules(pgVersion, true)))
}
