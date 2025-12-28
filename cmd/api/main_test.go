package main

import (
	"testing"

	v1 "github.com/pgconfig/api/cmd/api/handlers/v1"
)

func TestLoadConfiguration(t *testing.T) {
	// Paths relative to cmd/api
	rulesPath := "../../rules.yml"
	docsPath := "../../pg-docs.yml"

	// We still need to call LoadConfig because the API handlers depend on
	// allRules and pgDocs global variables being initialized.
	if err := v1.LoadConfig(rulesPath, docsPath); err != nil {
		t.Fatalf("Failed to load configuration: %v", err)
	}
}
