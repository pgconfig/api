package routes

import (
	"fmt"

	swagger "github.com/arsmn/fiber-swagger/v2"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/fiber/v2/middleware/recover"

	handV1 "github.com/pgconfig/api/cmd/api/handlers/v1"
)

// New create an instance of Book app routes
func New() *fiber.App {
	app := fiber.New(fiber.Config{
		// Override default error handler
		ErrorHandler: func(ctx *fiber.Ctx, err error) error {
			// Statuscode defaults to 500
			code := fiber.StatusInternalServerError

			// Retreive the custom statuscode if it's an fiber.*Error
			if e, ok := err.(*fiber.Error); ok {
				code = e.Code
			}

			if err != nil {
				// Send custom error page
				return ctx.Status(code).JSON(
					map[string]interface{}{
						"errors": map[string]interface{}{
							"code":    code,
							"message": err.Error(),
						},
						"links": map[string]interface{}{
							"self": fmt.Sprintf("%s%s", ctx.BaseURL(), ctx.OriginalURL()),
						},
						"jsonapi": map[string]interface{}{
							"version": "1.0",
						},
					})
			}

			// Return from handler
			return nil
		},
	})
	app.Use(cors.New())

	app.Use(recover.New(recover.Config{
		EnableStackTrace: true,
	}))
	app.Use(logger.New(logger.Config{
		Format:     "${cyan}[${time}] ${white}${pid} ${red}${status} ${blue}[${method}] ${white}${path}\n",
		TimeFormat: "02-Jan-2006 15:04:05",
		TimeZone:   "America/Sao_paulo",
	}))

	app.Get("/docs/*", swagger.Handler)
	app.Get("/docs", func(c *fiber.Ctx) error {
		return c.Redirect("/docs/index.html")
	})

	v1 := app.Group("/v1/tuning/")

	v1.Get("/list-environments", handV1.ListEnvs)
	v1.Get("/get-rules", handV1.GetRules)
	v1.Get("/get-config", handV1.GetConfig)
	v1.Get("/get-config-all-environments", handV1.GetConfigEnvs)

	return app
}
