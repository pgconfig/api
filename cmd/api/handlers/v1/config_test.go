package v1

import (
	"encoding/json"
	"io"
	"math"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/format"
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/input/profile"
	"github.com/pgconfig/api/pkg/rules"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestGetConfigPreservesLegacyResponses(t *testing.T) {
	if err := LoadConfig("../../../../rules.yml", "../../../../pg-docs.yml"); err != nil {
		t.Fatal(err)
	}

	tests := []struct {
		name            string
		query           string
		request         input.Input
		includePGBadger bool
		logFormat       string
	}{
		{
			name:      "defaults",
			query:     "",
			request:   defaultRESTInput(),
			logFormat: "stderr",
		},
		{
			name: "representative explicit request",
			query: "?pg_version=18.4&total_ram=64GB&max_connections=250" +
				"&environment_name=OLTP&os_type=windows&arch=386" +
				"&drive_type=SSD&cpus=16&include_pgbadger=true&log_format=jsonlog",
			request: input.Input{
				OS: "windows", Arch: "386", TotalRAM: 64 * bytes.GB,
				Profile: profile.OLTP, DiskType: "SSD", MaxConnections: 250,
				TotalCPU: 16, PostgresVersion: 18.4,
			},
			includePGBadger: true,
			logFormat:       "jsonlog",
		},
		{
			name:      "uppercase operating system keeps legacy tuning",
			query:     "?os_type=WINDOWS",
			request:   withRESTOperatingSystem(defaultRESTInput(), "WINDOWS"),
			logFormat: "stderr",
		},
		{
			name:      "lowercase storage keeps legacy tuning",
			query:     "?drive_type=ssd",
			request:   withRESTDiskType(defaultRESTInput(), "ssd"),
			logFormat: "stderr",
		},
		{
			name:      "zero PostgreSQL version keeps legacy response",
			query:     "?pg_version=0",
			request:   withRESTPostgreSQLVersion(defaultRESTInput(), 0),
			logFormat: "stderr",
		},
		{
			name:      "negative PostgreSQL version keeps legacy response",
			query:     "?pg_version=-1",
			request:   withRESTPostgreSQLVersion(defaultRESTInput(), -1),
			logFormat: "stderr",
		},
		{
			name:      "NaN PostgreSQL version keeps legacy response",
			query:     "?pg_version=NaN",
			request:   withRESTPostgreSQLVersion(defaultRESTInput(), float32(math.NaN())),
			logFormat: "stderr",
		},
		{
			name:      "infinite PostgreSQL version keeps legacy response",
			query:     "?pg_version=Inf",
			request:   withRESTPostgreSQLVersion(defaultRESTInput(), float32(math.Inf(1))),
			logFormat: "stderr",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			legacy, err := rules.Compute(tt.request)
			if err != nil {
				t.Fatal(err)
			}
			expected, err := json.Marshal(legacy.ToSlice(
				tt.request.PostgresVersion,
				tt.includePGBadger,
				tt.logFormat,
			))
			if err != nil {
				t.Fatal(err)
			}

			response := getConfigResponse(t, "/v1/tuning/get-config"+tt.query)
			if response.StatusCode != fiber.StatusOK {
				t.Fatalf("status = %d, want %d", response.StatusCode, fiber.StatusOK)
			}

			var body struct {
				Data    json.RawMessage `json:"data"`
				JSONAPI struct {
					Version string `json:"version"`
				} `json:"jsonapi"`
			}
			if err := json.NewDecoder(response.Body).Decode(&body); err != nil {
				t.Fatal(err)
			}
			if string(body.Data) != string(expected) {
				t.Fatalf("REST v1 data changed:\n got: %s\nwant: %s", body.Data, expected)
			}
			if body.JSONAPI.Version != "1.0" {
				t.Fatalf("JSON:API version = %q, want %q", body.JSONAPI.Version, "1.0")
			}
			if !containsParameter(body.Data, "listen_addresses") {
				t.Fatal("REST v1 response no longer contains listen_addresses")
			}
		})
	}
}

func TestGetConfigPreservesLegacyValidation(t *testing.T) {
	response := getConfigResponse(t, "/v1/tuning/get-config?arch=AMD64")
	if response.StatusCode != fiber.StatusInternalServerError {
		t.Fatalf("status = %d, want %d", response.StatusCode, fiber.StatusInternalServerError)
	}
}

func TestGetConfigPreservesLegacyFormats(t *testing.T) {
	request := defaultRESTInput()
	legacy, err := rules.Compute(request)
	if err != nil {
		t.Fatal(err)
	}
	legacyOutput := legacy.ToSlice(request.PostgresVersion, false, "stderr")

	for _, outputFormat := range []format.ExportFormat{format.Config, format.AlterSystemFormat} {
		t.Run(string(outputFormat), func(t *testing.T) {
			path := "/v1/tuning/get-config?format=" + string(outputFormat)
			expected := format.ExportConf(
				outputFormat,
				legacyOutput,
				request.PostgresVersion,
				[]string{"http://example.com" + path + "\n"},
			)

			response := getConfigResponse(t, path)
			actual, err := io.ReadAll(response.Body)
			if err != nil {
				t.Fatal(err)
			}
			if string(actual) != expected {
				t.Fatalf("REST v1 %s output changed:\n got: %s\nwant: %s", outputFormat, actual, expected)
			}
			if !strings.Contains(string(actual), "listen_addresses") {
				t.Fatal("REST v1 response no longer contains listen_addresses")
			}
		})
	}
}

func TestGetConfigPreservesDocumentationEnrichment(t *testing.T) {
	if err := LoadConfig("../../../../rules.yml", "../../../../pg-docs.yml"); err != nil {
		t.Fatal(err)
	}

	response := getConfigResponse(t, "/v1/tuning/get-config?show_doc=true")

	var body struct {
		Data []category.SliceOutput `json:"data"`
	}
	if err := json.NewDecoder(response.Body).Decode(&body); err != nil {
		t.Fatal(err)
	}
	for _, group := range body.Data {
		for _, parameter := range group.Parameters {
			if parameter.Name == "shared_buffers" {
				if parameter.Documentation == nil || parameter.Documentation.Title == "" {
					t.Fatal("shared_buffers documentation was not enriched")
				}
				return
			}
		}
	}
	t.Fatal("shared_buffers was not returned")
}

func defaultRESTInput() input.Input {
	return input.Input{
		OS: "linux", Arch: "amd64", TotalRAM: 2 * bytes.GB,
		Profile: profile.Web, DiskType: "HDD", MaxConnections: 100,
		TotalCPU: 2, PostgresVersion: 18,
	}
}

func withRESTOperatingSystem(request input.Input, operatingSystem string) input.Input {
	request.OS = operatingSystem
	return request
}

func withRESTDiskType(request input.Input, diskType string) input.Input {
	request.DiskType = diskType
	return request
}

func withRESTPostgreSQLVersion(request input.Input, postgresVersion float32) input.Input {
	request.PostgresVersion = postgresVersion
	return request
}

func getConfigResponse(t *testing.T, path string) *http.Response {
	t.Helper()
	app := fiber.New()
	app.Get("/v1/tuning/get-config", GetConfig)
	response, err := app.Test(httptest.NewRequest("GET", path, nil))
	if err != nil {
		t.Fatal(err)
	}
	return response
}

func containsParameter(data json.RawMessage, name string) bool {
	var categories []struct {
		Parameters []struct {
			Name string `json:"name"`
		} `json:"parameters"`
	}
	if err := json.Unmarshal(data, &categories); err != nil {
		return false
	}
	for _, category := range categories {
		for _, parameter := range category.Parameters {
			if parameter.Name == name {
				return true
			}
		}
	}
	return false
}

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
