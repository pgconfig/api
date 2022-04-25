package rules

import (
	"github.com/pgconfig/api/pkg/input"
	"github.com/pgconfig/api/pkg/input/bytes"
)

func fakeInput() *input.Input {
	return input.NewInput("linux", "amd64", 4*bytes.GB, 1, "WEB", "SSD", 100, 12.2)
}
