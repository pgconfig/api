package main

import (
	"log"

	"github.com/pgconfig/api/pkg/api"
	"github.com/gofiber/fiber"
)

const APIVersion = "/api/v2"

// Next with CORS
func next(c *fiber.Ctx) {
	c.Vary("Origin")     // => Vary: Origin
	c.Vary("User-Agent") // => Vary: Origin, User-Agent

	// No duplicates
	c.Vary("Origin") // => Vary: Origin, User-Agent
	c.Vary("Accept-Encoding", "Accept", "gzip", "deflate")
	c.Vary("Content-Type", "application/json")
	c.Next()
}

func main() {
	app := fiber.New()
	v2 := app.Group(APIVersion, next)
	api.SetupRoutesCompute(v2)
	if err:= app.Listen(3000); err != nil {
		log.Println("[ERR] not running API: ", err)
	}
}