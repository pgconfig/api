package v1

import (
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/input/profile"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestParseConfigArgs_ProfileCaseInsensitive(t *testing.T) {
	Describe("parseConfigArgs", t, func() {
		Context("profile parsing (issue #37)", func() {
			It("should accept environment_name case insensitively", func() {
				tests := []struct {
					name            string
					environmentName string
					expectedProfile profile.Profile
				}{
					{"uppercase MIXED", "MIXED", profile.Mixed},
					{"mixed case Mixed", "Mixed", profile.Mixed},
					{"lowercase mixed", "mixed", profile.Mixed},
					{"mixed case WEB", "Web", profile.Web},
					{"lowercase web", "web", profile.Web},
				}

				for _, tt := range tests {
					app := fiber.New()
					var capturedArgs *configArgs

					app.Get("/test", func(c *fiber.Ctx) error {
						args, err := parseConfigArgs(c)
						if err != nil {
							return err
						}
						capturedArgs = args
						return c.SendStatus(200)
					})

					req := httptest.NewRequest("GET", "/test?environment_name="+tt.environmentName, nil)
					_, err := app.Test(req)

					Expect(err, ShouldBeNil)
					Expect(capturedArgs, ShouldNotBeNil)
					Expect(capturedArgs.envName, ShouldEqual, tt.expectedProfile)
				}
			})
		})
	})
}
