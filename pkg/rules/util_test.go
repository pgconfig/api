package rules

import "github.com/pgconfig/api/pkg/config"

func fakeInput() *config.Input {
	return config.NewInput("linux", "x86_64", 4*config.GB, "WEB", "SSD", 100, 12.2)
}
