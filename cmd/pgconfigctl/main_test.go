package main

import (
	"bytes"
	"os"
	"strings"
	"testing"

	"github.com/pgconfig/api/cmd/pgconfigctl/cmd"
)

// TestTuneProfileParsing validates that all profile types parse correctly
// This addresses issue #22 where Mixed and Desktop profiles were rejected
func TestTuneProfileParsing(t *testing.T) {
	tests := []struct {
		name      string
		args      []string
		wantError bool
		errorMsg  string
	}{
		{
			name:      "Mixed profile - mixed case (issue #22)",
			args:      []string{"tune", "--profile=Mixed"},
			wantError: false,
		},
		{
			name:      "Mixed profile - uppercase",
			args:      []string{"tune", "--profile=MIXED"},
			wantError: false,
		},
		{
			name:      "Mixed profile - lowercase",
			args:      []string{"tune", "--profile=mixed"},
			wantError: false,
		},
		{
			name:      "Desktop profile - mixed case (issue #22)",
			args:      []string{"tune", "--profile=Desktop"},
			wantError: false,
		},
		{
			name:      "Desktop profile - uppercase",
			args:      []string{"tune", "--profile=DESKTOP"},
			wantError: false,
		},
		{
			name:      "Desktop profile - lowercase",
			args:      []string{"tune", "--profile=desktop"},
			wantError: false,
		},
		{
			name:      "Web profile - uppercase",
			args:      []string{"tune", "--profile=WEB"},
			wantError: false,
		},
		{
			name:      "Web profile - lowercase",
			args:      []string{"tune", "--profile=web"},
			wantError: false,
		},
		{
			name:      "OLTP profile",
			args:      []string{"tune", "--profile=OLTP"},
			wantError: false,
		},
		{
			name:      "DW profile",
			args:      []string{"tune", "--profile=DW"},
			wantError: false,
		},
		{
			name:      "Invalid profile",
			args:      []string{"tune", "--profile=invalid"},
			wantError: true,
			errorMsg:  "must be one of",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Capture output
			outBuf := new(bytes.Buffer)
			errBuf := new(bytes.Buffer)

			// Save and restore original output
			originalOut := os.Stdout
			originalErr := os.Stderr
			t.Cleanup(func() {
				os.Stdout = originalOut
				os.Stderr = originalErr
			})

			// Set command output
			cmd.RootCmd.SetOut(outBuf)
			cmd.RootCmd.SetErr(errBuf)
			cmd.RootCmd.SetArgs(tt.args)

			// Execute command
			err := cmd.RootCmd.Execute()

			if tt.wantError {
				if err == nil {
					t.Errorf("Expected error but got none")
					return
				}
				if tt.errorMsg != "" && !strings.Contains(err.Error(), tt.errorMsg) {
					t.Errorf("Error message = %v, want to contain %v", err.Error(), tt.errorMsg)
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error: %v\nOutput: %s\nError output: %s",
						err, outBuf.String(), errBuf.String())
				}
			}
		})
	}
}
