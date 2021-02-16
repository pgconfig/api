package v1

import (
	"bytes"
	"fmt"
	"strconv"
	"strings"
	"unicode"

	"github.com/gofiber/fiber/v2"

	"github.com/pgconfig/api/pkg/category"
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/defaults"
	"github.com/pgconfig/api/pkg/docs"
	"github.com/pgconfig/api/pkg/format"
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
// @Param show_doc query string false "Show Documentation args" Enums(true,false) default(false)
// @Param include_pgbadger query string false "Add pgbadger configuration" Enums(true,false) default(false)
// @Param log_format query string false "Defines the log_format to be used" Enums(stderr,csvlog,syslog) default(stderr)
// @Success 200 {object} ResponseHTTP{}
// @Router /v1/tuning/get-config [get]
func GetConfig(c *fiber.Ctx) error {

	args, err := parseConfigArgs(c)

	if err != nil {
		return fmt.Errorf("could not parse args: %w", err)
	}

	finalData, err := processConfig(c, args)

	if err != nil {
		return fmt.Errorf("could not process config: %w", err)
	}

	switch args.outFormat {
	case "alter_system":
		return c.SendString(formatConf(c, args.outFormat, finalData))
	case "conf":
		return c.SendString(formatConf(c, args.outFormat, finalData))
	default:
		return c.JSON(v1Reponse(c, finalData))
	}
}

func processConfig(c *fiber.Ctx, args *configArgs) ([]category.SliceOutput, error) {
	input := *config.NewInput(
		args.osType,
		args.arch,
		args.totalRAM,
		args.cpuCount,
		args.envName,
		args.driveType,
		args.maxConn,
		args.pgVersion)

	tune, err := rules.Compute(input)

	if err != nil {
		return nil, err
	}

	output := tune.ToSlice(args.pgVersion, args.includePgbadger, args.logFormat)

	if args.showDoc {
		doc := pgDocs.Documentation[docs.FormatVer(args.pgVersion)]

		for c := 0; c < len(output); c++ {

			rules := allRules.Categories[output[c].Name]
			for p := 0; p < len(output[c].Parameters); p++ {
				paramDocs := doc[output[c].Parameters[p].Name]
				paramRule := rules[output[c].Parameters[p].Name]

				output[c].Parameters[p].Documentation = &docs.ParamDoc{
					Title:              paramDocs.Title,
					ShortDesc:          paramDocs.ShortDesc,
					Text:               paramDocs.Text,
					DocURL:             paramDocs.DocURL,
					ConfURL:            paramDocs.ConfURL,
					RecomendationsConf: paramDocs.RecomendationsConf,
					ParamType:          paramDocs.ParamType,
					DefaultValue:       paramDocs.DefaultValue,
					MinValue:           paramDocs.MinValue,
					MaxValue:           paramDocs.MaxValue,
					BlogRecomendations: paramRule.Recomendations,
					Abstract:           paramRule.Abstract,
				}

			}
		}
	}

	return output, nil

}

func parseConfigArgs(c *fiber.Ctx) (*configArgs, error) {

	pgVersion, err := strconv.ParseFloat(c.Query("pg_version", defaults.PGVersion), 32)

	if err != nil {
		return nil, err
	}
	maxConn, err := strconv.Atoi(c.Query("max_connections", "100"))

	if err != nil {
		return nil, err
	}

	cpuCount, err := strconv.Atoi(c.Query("cpus", "2"))

	if err != nil {
		return nil, err
	}

	return &configArgs{
		pgVersion:       float32(pgVersion),
		totalRAM:        parseRAM(strings.ToUpper(c.Query("total_ram", "2GB"))),
		maxConn:         maxConn,
		envName:         c.Query("environment_name", "WEB"),
		osType:          c.Query("os_type", "linux"),
		arch:            c.Query("arch", "amd64"),
		driveType:       c.Query("drive_type", "HDD"),
		cpuCount:        cpuCount,
		outFormat:       c.Query("format", "json"),
		showDoc:         c.Query("show_doc", "false") == "true",
		includePgbadger: c.Query("include_pgbadger", "false") == "true",
		logFormat:       c.Query("log_format", "stderr"),
	}, nil
}

type configArgs struct {
	pgVersion       float32
	totalRAM        config.Byte
	maxConn         int
	envName         string
	osType          string
	arch            string
	driveType       string
	cpuCount        int
	outFormat       string
	showDoc         bool
	includePgbadger bool
	logFormat       string
}

func formatConf(c *fiber.Ctx, f string, output []category.SliceOutput) string {

	var comment string

	switch f {
	case "alter_system":
		comment = "--"
	default:
		comment = "#"
	}

	var b bytes.Buffer

	b.WriteString(fmt.Sprintf("%s Generated by PGConfig 3.0 alpha\n", fillComment(1, comment)))
	b.WriteString(fmt.Sprintf("%s %s%s\n", fillComment(1, comment), c.BaseURL(), c.OriginalURL()))
	b.WriteString("\n")

	if f == "alter_system" {
		b.WriteString(format.AlterSystem(output))
		return b.String()
	}

	b.WriteString(format.ConfigFile(output))
	return b.String()
}

func fillComment(qtd int, comment string) string {
	var b bytes.Buffer

	for i := 0; i < qtd; i++ {
		b.WriteString(comment)
	}

	return b.String()
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
