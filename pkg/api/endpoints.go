package api

import (
	"log"

	"github.com/pgconfig/api/pkg/compute"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/gofiber/fiber"
)

func SetupRoutesCompute(rtr fiber.Router) {
	g := rtr.Group("/compare")

	// Setup routes
	g.Post("/", compare)
}

func compare(ctx *fiber.Ctx) {
	var in *config.Input
	if err := ctx.BodyParser(&in); err != nil {
		if err := ctx.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"errors": errors.ErrorInvalidSchema.Error(),
		}); err != nil {
			log.Println("Staus, JSON -> Error CTX fiber", err)
		}
		return
	}

	v := newValidator()
	v.validInputs(*in)
	if v.hasErrors() {
		if err := ctx.Status(fiber.StatusUnprocessableEntity).JSON(fiber.Map{
			"errors": v.Errors,
		}); err != nil {
			log.Println("v.hasErrors() -> Error CTX fiber", err)
		}
		return  
	}

	// TODO: return compute
	// *config.Input, *category.ExportCfg, error
	cIn, cExC, err := compute.Compute(*in) 
	if err != nil {
		log.Println("compute.Compute(*in) -> ", err)
		ctx.Status(fiber.StatusUnprocessableEntity).JSON(fiber.Map{
			"errors": v.Errors,
		})
		return
	}

	ctx.Status(200).JSON(fiber.Map{
		"Input": cIn,
		"ExportCfg": cExC,
	})
}