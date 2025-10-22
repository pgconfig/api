package defaults

const (
	// PGVersion is the current stable version of PostgreSQL
	PGVersion = "16"

	// PGVersionF is the current stable version of PostgreSQL - on the float Format
	PGVersionF = 16.0
)

var (

	// SupportedVersions is the list of supported versions
	SupportedVersions = []float32{
		9.1,
		9.2,
		9.3,
		9.4,
		9.5,
		9.6,
		10.0,
		11.0,
		12.0,
		13.0,
		14.0,
		15.0,
		16.0,
		17.0,
	}
)
