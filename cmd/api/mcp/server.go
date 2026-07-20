// Package mcp exposes the tuning operation through the Model Context Protocol.
package mcp

import (
	"context"
	"net/http"

	mcpsdk "github.com/modelcontextprotocol/go-sdk/mcp"
	"github.com/pgconfig/api/pkg/rules"
	"github.com/pgconfig/api/pkg/version"
)

const recommendToolName = "recommend_postgres_configuration"

// NewHandler returns the stateless Streamable HTTP MCP endpoint.
func NewHandler() http.Handler {
	server := mcpsdk.NewServer(&mcpsdk.Implementation{
		Name:    "pgconfig",
		Version: version.Pretty(),
	}, nil)

	falseValue := false
	mcpsdk.AddTool(server, &mcpsdk.Tool{
		Name: recommendToolName,
		Description: "Return deterministic PostgreSQL configuration " +
			"recommendations for a complete tuning request.",
		Annotations: &mcpsdk.ToolAnnotations{
			DestructiveHint: &falseValue,
			IdempotentHint:  true,
			OpenWorldHint:   &falseValue,
			ReadOnlyHint:    true,
		},
	}, recommendPostgresConfiguration)

	return mcpsdk.NewStreamableHTTPHandler(
		func(*http.Request) *mcpsdk.Server { return server },
		&mcpsdk.StreamableHTTPOptions{
			Stateless:    true,
			JSONResponse: true,
		},
	)
}

func recommendPostgresConfiguration(
	_ context.Context,
	_ *mcpsdk.CallToolRequest,
	request rules.TuningRequest,
) (*mcpsdk.CallToolResult, rules.TuningResult, error) {
	result, err := rules.Tune(request)
	if err != nil {
		return nil, rules.TuningResult{}, err
	}
	return nil, *result, nil
}
