package v1

import (
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/input/profile"
)

func TestParseConfigArgs_ProfileCaseInsensitive(t *testing.T) {
	// Regression test for issue #37: environment_name should be case-insensitive
	tests := []struct {
		name            string
		environmentName string
		expectedProfile profile.Profile
	}{
		{
			name:            "uppercase MIXED",
			environmentName: "MIXED",
			expectedProfile: profile.Mixed,
		},
		{
			name:            "mixed case Mixed",
			environmentName: "Mixed",
			expectedProfile: profile.Mixed,
		},
		{
			name:            "lowercase mixed",
			environmentName: "mixed",
			expectedProfile: profile.Mixed,
		},
		{
			name:            "mixed case WEB",
			environmentName: "Web",
			expectedProfile: profile.Web,
		},
		{
			name:            "lowercase web",
			environmentName: "web",
			expectedProfile: profile.Web,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			app := fiber.New()

			app.Get("/test", func(c *fiber.Ctx) error {
				args, err := parseConfigArgs(c)
				if err != nil {
					return err
				}

				if args.envName != tt.expectedProfile {
					t.Errorf("expected profile %v, got %v", tt.expectedProfile, args.envName)
				}

				return c.SendStatus(200)
			})

			req := httptest.NewRequest("GET", "/test?environment_name="+tt.environmentName, nil)
			_, err := app.Test(req)
			if err != nil {
				t.Fatalf("request failed: %v", err)
			}
		})
	}
}
