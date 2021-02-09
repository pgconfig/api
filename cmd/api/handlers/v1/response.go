package v1

import (
	"fmt"
	"net/url"

	"github.com/gofiber/fiber/v2"
)

func v1Reponse(c *fiber.Ctx, data interface{}) ResponseHTTP {
	u, _ := url.Parse(c.OriginalURL())
	queryParams := u.Query()

	return ResponseHTTP{
		Data:    data,
		Jsonapi: jsonapi{Version: "1.0"},
		Links: links{
			Self: fmt.Sprintf("%s%s", c.BaseURL(), c.OriginalURL()),
		},
		Meta: meta{
			Copyright: "PGConfig API",
			Version:   "2.0 beta",
			Arguments: queryParams,
		},
	}
}

// ResponseHTTP represents response body of this API
type ResponseHTTP struct {
	Data    interface{} `json:"data"`
	Jsonapi jsonapi     `json:"jsonapi"`
	Links   links       `json:"links"`
	Meta    meta        `json:"meta"`
}
type jsonapi struct {
	Version string `json:"version"`
}
type links struct {
	Self string `json:"self"`
}
type meta struct {
	Arguments interface{} `json:"arguments,omitempty"`
	Copyright string      `json:"copyright"`
	Version   string      `json:"version"`
}
