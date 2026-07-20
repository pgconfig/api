package routes_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"sync"
	"testing"

	"github.com/gofiber/fiber/v2/middleware/adaptor"
	"github.com/modelcontextprotocol/go-sdk/mcp"
	"github.com/pgconfig/api/cmd/api/routes"
	"github.com/pgconfig/api/pkg/input/bytes"
	"github.com/pgconfig/api/pkg/version"
	"github.com/stretchr/testify/require"
)

func TestMCPHappyPath(t *testing.T) {
	app := routes.New()
	httpServer := httptest.NewServer(adaptor.FiberApp(app))
	t.Cleanup(httpServer.Close)

	ctx := context.Background()
	recorder := &responseRecorder{base: httpServer.Client().Transport}
	client := mcp.NewClient(&mcp.Implementation{Name: "pgconfig-test", Version: "1.0.0"}, nil)
	session, err := client.Connect(ctx, &mcp.StreamableClientTransport{
		Endpoint:             httpServer.URL + "/mcp",
		HTTPClient:           &http.Client{Transport: recorder},
		DisableStandaloneSSE: true,
	}, nil)
	require.NoError(t, err)
	t.Cleanup(func() { require.NoError(t, session.Close()) })

	initializeResult := session.InitializeResult()
	require.Equal(t, "pgconfig", initializeResult.ServerInfo.Name)
	require.Equal(t, version.Pretty(), initializeResult.ServerInfo.Version)

	tools, err := session.ListTools(ctx, nil)
	require.NoError(t, err)
	require.Len(t, tools.Tools, 1)

	tool := tools.Tools[0]
	require.Equal(t, "recommend_postgres_configuration", tool.Name)
	require.Contains(t, tool.Description, "deterministic")
	require.NotNil(t, tool.OutputSchema)
	require.NotNil(t, tool.Annotations)
	require.True(t, tool.Annotations.ReadOnlyHint)
	require.True(t, tool.Annotations.IdempotentHint)
	require.NotNil(t, tool.Annotations.DestructiveHint)
	require.False(t, *tool.Annotations.DestructiveHint)
	require.NotNil(t, tool.Annotations.OpenWorldHint)
	require.False(t, *tool.Annotations.OpenWorldHint)

	callResult, err := session.CallTool(ctx, &mcp.CallToolParams{
		Name: tool.Name,
		Arguments: map[string]any{
			"os":               " Linux ",
			"arch":             " x86-64 ",
			"total_ram":        int64(16 * bytes.GB),
			"profile":          " web ",
			"disk_type":        " ssd ",
			"max_connections":  100,
			"total_cpu":        8,
			"postgres_version": " 18.4 ",
		},
	})
	require.NoError(t, err)
	require.False(t, callResult.IsError)
	require.Len(t, callResult.Content, 1)

	structured, ok := callResult.StructuredContent.(map[string]any)
	require.True(t, ok)
	request := structured["request"].(map[string]any)
	require.Equal(t, "linux", request["os"])
	require.Equal(t, "amd64", request["arch"])
	require.Equal(t, "WEB", request["profile"])
	require.Equal(t, "SSD", request["disk_type"])
	require.Equal(t, "18.4", request["postgres_version"])
	require.Empty(t, structured["assumptions"])
	require.Empty(t, structured["warnings"])

	recommendations := structured["recommendations"].(map[string]any)
	require.NotEmpty(t, recommendations)
	require.NotContains(t, recommendations, "listen_addresses")

	text, ok := callResult.Content[0].(*mcp.TextContent)
	require.True(t, ok)
	structuredJSON, err := json.Marshal(structured)
	require.NoError(t, err)
	require.JSONEq(t, string(structuredJSON), text.Text)

	contentTypes := recorder.successfulPostContentTypes()
	require.Len(t, contentTypes, 3)
	for _, contentType := range contentTypes {
		require.Equal(t, "application/json", contentType)
	}

	requestWithoutSession, err := http.NewRequest(http.MethodGet, httpServer.URL+"/mcp", nil)
	require.NoError(t, err)
	requestWithoutSession.Header.Set("Accept", "text/event-stream")
	response, err := httpServer.Client().Do(requestWithoutSession)
	require.NoError(t, err)
	t.Cleanup(func() { require.NoError(t, response.Body.Close()) })
	require.Equal(t, http.StatusMethodNotAllowed, response.StatusCode)
	require.Equal(t, "POST", response.Header.Get("Allow"))
}

type responseRecorder struct {
	base         http.RoundTripper
	mu           sync.Mutex
	contentTypes []string
}

func (r *responseRecorder) RoundTrip(request *http.Request) (*http.Response, error) {
	response, err := r.base.RoundTrip(request)
	if err == nil && request.Method == http.MethodPost && response.StatusCode == http.StatusOK {
		r.mu.Lock()
		r.contentTypes = append(r.contentTypes, response.Header.Get("Content-Type"))
		r.mu.Unlock()
	}
	return response, err
}

func (r *responseRecorder) successfulPostContentTypes() []string {
	r.mu.Lock()
	defer r.mu.Unlock()
	return append([]string(nil), r.contentTypes...)
}
