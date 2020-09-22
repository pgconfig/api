package main

import (
	"flag"
	"log"

	"github.com/gofiber/fiber"
	"github.com/pgconfig/api/pkg/api"
	"github.com/pgconfig/api/pkg/version"
)

const APIVersion = "/api/v2"

var port int

func init() {
	flag.IntVar(&port, "version", 3000, "Listen port")
	flag.Parse()
}

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
	log.Printf("%s (%s)\n", version.Tag, version.Commit)
	app := fiber.New()
	v2 := app.Group(APIVersion, next)
	api.SetupRoutesCompute(v2)
	if err := app.Listen(port); err != nil {
		log.Println("[ERR] not running API: ", err)
	}
}
