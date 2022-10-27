package docs

import (
	"fmt"
	"log"
	"net/http"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

// ParamDoc is foo
type ParamDoc struct {
	Title              string            `json:"name,omitempty" yaml:"title"`
	ShortDesc          string            `json:"short_desc,omitempty" yaml:"short_desc"`
	Text               []string          `json:"details,omitempty" yaml:"details"`
	DocURL             string            `json:"url,omitempty" yaml:"url"`
	ConfURL            string            `json:"conf_url,omitempty" yaml:"conf_url"`
	RecomendationsConf string            `json:"recomendations_conf,omitempty" yaml:"recomendations_conf"`
	ParamType          string            `json:"type,omitempty" yaml:"type"`
	DefaultValue       string            `json:"default_value,omitempty" yaml:"default_value"`
	MinValue           string            `json:"min_value,omitempty" yaml:"min_value"`
	MaxValue           string            `json:"max_value,omitempty" yaml:"max_value"`
	BlogRecomendations map[string]string `json:"recomendations,omitempty,omitempty" yaml:"recomendations,omitempty"`
	Abstract           string            `json:"abstract,omitempty" yaml:"abstract,omitempty"`
}

// FormatVer fixes the postgres versioning system and results a valid version
func FormatVer(ver float32) string {
	if ver < 10 {
		return fmt.Sprintf("%.1f", ver)
	}

	return fmt.Sprintf("%.0f", ver)
}

// Get queries conf website and gets the html from the webpage and parses it
func Get(param string, ver float32) (ParamDoc, error) {

	var out ParamDoc
	out.ConfURL = fmt.Sprintf("https://postgresqlco.nf/en/doc/param/%s/%s/", param, FormatVer(ver))

	res, err := http.Get(out.ConfURL)

	if err != nil {
		return out, fmt.Errorf("could not get URL: %v", err)
	}
	defer res.Body.Close()

	if res.StatusCode != 200 {
		return out, fmt.Errorf("status code error: %d %s", res.StatusCode, res.Status)
	}

	doc, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		log.Fatal(err)
	}

	// title
	sel := doc.Find("body > div.wrapper > div > section.content-header > div > div.col-md-8 > h1.parameter-title")
	for i := range sel.Nodes {

		sel.Eq(i).Children().Remove()

		out.Title = t(sel.Eq(i).Text())
	}

	// type
	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div.box.box-info > div > table > tbody > tr:nth-child(1) > td:nth-child(2) > code")

	for i := range sel.Nodes {

		finalType := t(sel.Eq(i).Text())

		if finalType == "real" {
			out.ParamType = "floating point"
			continue
		}

		out.ParamType = finalType
	}

	// short desc
	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div.box.box-solid.box-primary > div:nth-child(1) > strong")
	for i := range sel.Nodes {
		out.ShortDesc = t(sel.Eq(i).Text())
	}

	// doc text
	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div.box.box-solid.box-primary > div.box-body > p")
	for i := range sel.Nodes {

		out.Text = append(out.Text, t(sel.Eq(i).Text()))
	}

	// doc url?
	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div.box.box-solid.box-primary > div:nth-child(3) > span:nth-child(1) > a")
	for i := range sel.Nodes {
		single, e := sel.Eq(i).Attr("href")

		if e {
			out.DocURL = single
		}
	}

	// recomendations
	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div:nth-child(3) > div.box-header.with-border > h4")
	previousSession := ""
	for i := range sel.Nodes {
		previousSession = t(sel.Eq(i).Text())
	}

	sel = doc.Find("body > div.wrapper > div > section.content > div > div.col-md-8 > div:nth-child(3) > div.box-body")
	for i := range sel.Nodes {
		if previousSession == "Recommendations" {
			out.RecomendationsConf = t(sel.Eq(i).Text())
		}
	}

	// default values
	sel = doc.Find("div.box-body:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > code:nth-child(1)")
	for i := range sel.Nodes {
		out.DefaultValue = sanitizeDefault(t(sel.Eq(i).Text()))
	}

	// min values
	sel = doc.Find("div.box-body:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(2) > code:nth-child(1)")
	for i := range sel.Nodes {
		out.MinValue = sanitizeDefault(t(sel.Eq(i).Text()))
	}

	// max values
	sel = doc.Find("div.box-body:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(4) > td:nth-child(2) > code:nth-child(1)")
	for i := range sel.Nodes {
		out.MaxValue = sanitizeDefault(t(sel.Eq(i).Text()))
	}

	return out, nil
}

func t(i string) string {
	return strings.TrimSpace(i)
}

func sanitizeDefault(val string) string {

	parts := strings.Fields(val)

	if len(parts) > 1 {
		if strings.HasPrefix(parts[1], "(") {
			return strings.ReplaceAll(
				strings.ReplaceAll(parts[1], "(", ""),
				")",
				"",
			)
		}
	}

	return val
}
