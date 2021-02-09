package docs

// Doc is a map with parameters
type Doc map[string]ParamDoc

// DocFile is the json/yaml structure of the doc file
type DocFile struct {
	Documentation map[string]Doc `json:"documentation"`
}
