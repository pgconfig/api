package rules

import "github.com/pgconfig/api/pkg/config"

func fakeInput() *config.Input {
	return config.NewInput("linux", "amd64", 4*config.GB, 1, "WEB", "SSD", 100, 12.2)
}
