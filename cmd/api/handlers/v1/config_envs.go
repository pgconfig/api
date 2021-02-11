package v1

import (
	"fmt"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/profile"
)

// GetConfigEnvs is a function to that computes the input and suggests a tuning configuration for all supported profiles
// @Summary Get Configuration
// @Description computes the input and suggests a tuning configuration
// @Accept json
// @Produce json
// @Param pg_version query string false "PostgreSQL Version" default(13)
// @Param total_ram query string false "Total dedicated memory to PostgreSQL" default(2GB)
// @Param max_connections query integer false "Total expected number of connections" default(100)
// @Param os_type query string false "Type of operating system used" Enums(linux,windows,unix) default(linux)
// @Param arch query string false "server architecture" Enums(386,amd64,arm,arm64) default(amd64)
// @Param drive_type query string false "default storage type" Enums(HDD,SSD,SAN) default(HDD)
// @Param cpus query integer false "Total CPUs available" default(2)
// @Param format query string false "Output format" Enums(json) default(json)
// @Param show_doc query string false "Show Documentation args" Enums(true,false) default(false)
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-config-all-environments [get]
func GetConfigEnvs(c *fiber.Ctx) error {

	args, err := parseConfigArgs(c)

	if err != nil {
		return fmt.Errorf("could not parse args: %w", err)
	}

	args.includePgbadger = false

	var out []allEnvsOutput

	for _, env := range profile.AllProfiles {
		args.envName = env
		finalData, err := processConfig(c, args)

		if err != nil {
			return fmt.Errorf("could not process config: %w", err)
		}

		out = append(out, allEnvsOutput{
			EnvName: env,
			Config:  finalData,
		})
	}

	return c.JSON(v1Reponse(c, out))
}

type allEnvsOutput struct {
	EnvName string           `json:"environment"`
	Config  []outputCategory `json:"configuration"`
}
