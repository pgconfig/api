package main

import (
	"bytes"
	"os"
	"testing"

	"github.com/pgconfig/api/cmd/pgconfigctl/cmd"
	. "github.com/pgconfig/api/pkg/tests"
)

func TestTuneProfileParsing(t *testing.T) {
	Describe("tune command", t, func() {
		Context("profile parsing (issue #22)", func() {
			It("should accept all profile types case insensitively", func() {
				tests := []struct {
					name      string
					args      []string
					wantError bool
					errorMsg  string
				}{
					{"Mixed profile - mixed case", []string{"tune", "--profile=Mixed"}, false, ""},
					{"Mixed profile - uppercase", []string{"tune", "--profile=MIXED"}, false, ""},
					{"Mixed profile - lowercase", []string{"tune", "--profile=mixed"}, false, ""},
					{"Desktop profile - mixed case", []string{"tune", "--profile=Desktop"}, false, ""},
					{"Desktop profile - uppercase", []string{"tune", "--profile=DESKTOP"}, false, ""},
					{"Desktop profile - lowercase", []string{"tune", "--profile=desktop"}, false, ""},
					{"Web profile - uppercase", []string{"tune", "--profile=WEB"}, false, ""},
					{"Web profile - lowercase", []string{"tune", "--profile=web"}, false, ""},
					{"OLTP profile", []string{"tune", "--profile=OLTP"}, false, ""},
					{"DW profile", []string{"tune", "--profile=DW"}, false, ""},
					{"Invalid profile", []string{"tune", "--profile=invalid"}, true, "must be one of"},
				}

				for _, tt := range tests {
					// Capture output
					outBuf := new(bytes.Buffer)
					errBuf := new(bytes.Buffer)

					// Save and restore original output
					originalOut := os.Stdout
					originalErr := os.Stderr
					defer func() {
						os.Stdout = originalOut
						os.Stderr = originalErr
					}()

					// Set command output
					cmd.RootCmd.SetOut(outBuf)
					cmd.RootCmd.SetErr(errBuf)
					cmd.RootCmd.SetArgs(tt.args)

					// Execute command
					err := cmd.RootCmd.Execute()

					if tt.wantError {
						Expect(err, ShouldNotBeNil)
						if tt.errorMsg != "" {
							Expect(err.Error(), ShouldContainSubstring, tt.errorMsg)
						}
					} else {
						Expect(err, ShouldBeNil)
					}
				}
			})
		})
	})
}
