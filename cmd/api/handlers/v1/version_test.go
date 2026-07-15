package v1

import (
	"encoding/json"
	"io"
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/version"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestVersion(t *testing.T) {
	Describe("Version handler", t, func() {
		Context("GET /v1/version", func() {
			tests := []struct {
				name     string
				tag      string
				commit   string
				expected string
			}{
				{
					name:     "release build",
					tag:      "3.5.0",
					commit:   "0489fd75bc6a990acb14747f5d01d8447c194152",
					expected: "3.5.0 (0489fd75bc6a990acb14747f5d01d8447c194152)",
				},
				{
					name:     "development build",
					tag:      "development",
					commit:   "latest",
					expected: "development (latest)",
				},
			}

			for _, tt := range tests {
				It("returns 200 with version envelope for "+tt.name, func() {
					origTag := version.Tag
					origCommit := version.Commit
					version.Tag = tt.tag
					version.Commit = tt.commit
					defer func() {
						version.Tag = origTag
						version.Commit = origCommit
					}()

					app := fiber.New()
					app.Get("/v1/version", Version)

					req := httptest.NewRequest("GET", "/v1/version", nil)
					resp, err := app.Test(req)
					Expect(err, ShouldBeNil)
					Expect(resp.StatusCode, ShouldEqual, 200)
					Expect(resp.Header.Get("Cache-Control"), ShouldEqual, "public, max-age=3600")

					body, err := io.ReadAll(resp.Body)
					Expect(err, ShouldBeNil)

					var envelope struct {
						Data struct {
							Version string `json:"version"`
							Build   string `json:"build"`
							Pretty  string `json:"pretty"`
						} `json:"data"`
						Jsonapi struct {
							Version string `json:"version"`
						} `json:"jsonapi"`
						Meta struct {
							Copyright string `json:"copyright"`
							Version   string `json:"version"`
						} `json:"meta"`
					}
					err = json.Unmarshal(body, &envelope)
					Expect(err, ShouldBeNil)
					Expect(envelope.Data.Version, ShouldEqual, tt.tag)
					Expect(envelope.Data.Build, ShouldEqual, tt.commit)
					Expect(envelope.Data.Pretty, ShouldEqual, tt.expected)
					Expect(envelope.Jsonapi.Version, ShouldEqual, "1.0")
					Expect(envelope.Meta.Copyright, ShouldEqual, "PGConfig API")
					Expect(envelope.Meta.Version, ShouldEqual, tt.expected)
				})
			}
		})
	})
}
