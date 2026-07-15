package v1

import (
	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/version"
)

type versionData struct {
	Version string `json:"version"`
	Build   string `json:"build"`
	Pretty  string `json:"pretty"`
}

// Version returns API build version information.
// @Summary Get API version
// @Description returns the current API version and build metadata
// @Accept json
// @Produce json
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/version [get]
func Version(c *fiber.Ctx) error {
	c.Set("Cache-Control", "public, max-age=3600")

	return c.JSON(v1Reponse(c, versionData{
		Version: version.Tag,
		Build:   version.Commit,
		Pretty:  version.Pretty(),
	}))
}
