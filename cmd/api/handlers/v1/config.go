package v1

import (
	"bytes"
	"strconv"
	"strings"
	"unicode"

	"github.com/gofiber/fiber/v2"

	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/rules"
)

// GetConfig is a function to that computes the input and suggests a tuning configuration
// @Summary Get Configuration
// @Description computes the input and suggests a tuning configuration
// @Accept json
// @Produce json
// @Param pg_version query string false "PostgreSQL Version" default(13)
// @Param total_ram query string false "Total dedicated memory to PostgreSQL" default(2GB)
// @Param max_connections query integer false "Total expected number of connections" default(100)
// @Param environment_name query string false "Application profile of the server" Enums(WEB,OLTP,DW,Mixed,Desktop) default(WEB)
// @Param os_type query string false "Type of operating system used" Enums(linux,windows,unix) default(linux)
// @Param arch query string false "server architecture" Enums(386,amd64,arm,arm64) default(amd64)
// @Param drive_type query string false "default storage type" Enums(HDD,SSD,SAN) default(HDD)
// @Param cpus query integer false "Total CPUs available" default(2)
// @Param format query string false "Output format" Enums(json,alter_system,conf) default(json)
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-config [get]
func GetConfig(c *fiber.Ctx) error {

	cpuCount, err := strconv.Atoi(c.Query("cpus", "2"))

	if err != nil {
		return err
	}
	maxConn, err := strconv.Atoi(c.Query("max_connections", "100"))

	if err != nil {
		return err
	}
	pgVersion, err := strconv.ParseFloat(c.Query("pg_version", "13"), 32)

	if err != nil {
		return err
	}

	input := *config.NewInput(
		c.Query("os_type", "linux"),
		c.Query("arch", "amd64"),
		parseRAM(strings.ToUpper(c.Query("total_ram", "2GB"))),
		cpuCount,
		c.Query("environment_name", "WEB"),
		c.Query("drive_type", "HDD"),
		maxConn,
		float32(pgVersion),
	)

	out, err := rules.Compute(input)

	if err != nil {
		return err
	}

	// todo: merge value with the yaml rules
	// todo: merge with docs info
	return c.JSON(v1Reponse(c, out))
}

func parseRAM(compared string) config.Byte {

	val := extractNumber([]rune(compared))

	switch {
	case strings.HasSuffix(compared, "KB"):
		return val * config.KB
	case strings.HasSuffix(compared, "MB"):
		return val * config.MB
	case strings.HasSuffix(compared, "GB"):
		return val * config.GB
	case strings.HasSuffix(compared, "TB"):
		return val * config.TB
	default:
		return val
	}
}

func extractNumber(val []rune) config.Byte {

	var b bytes.Buffer

	for i := 0; i < len(val); i++ {
		if unicode.IsNumber(val[i]) {
			b.WriteRune(val[i])
		}
	}

	num, err := strconv.Atoi(b.String())

	if err != nil {
		panic(err)
	}

	return config.Byte(num)
}
