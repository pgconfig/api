package profile

const (

	// Web profile
	Web = "WEB"

	// OLTP profile
	OLTP = "OLTP"

	// DW profile
	DW = "DW"

	// Mixed profile
	Mixed = "Mixed"

	// Desktop is the development machine on any non-production server
	// that needs to consume less resources than a regular server.
	Desktop = "Desktop"
)

// AllProfiles Lists all profiles currently available
var AllProfiles = []string{Web, OLTP, DW, Mixed, Desktop}
