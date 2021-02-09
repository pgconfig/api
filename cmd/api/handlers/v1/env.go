package v1

import (
	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/profile"
)

// ListEnvs is a function to list all environments
// @Summary Lists all environments
// @Description list all supported environment profiles
// @Accept json
// @Produce json
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/list-environments [get]
func ListEnvs(c *fiber.Ctx) error {
	return c.JSON(v1Reponse(c, profile.AllProfiles))
}
