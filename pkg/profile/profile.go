package profile

const (
	Web   = "WEB"
	OLTP  = "OLTP"
	DW    = "DW"
	Mixed = "Mixed"

	// Desktop is the development machine on any non-production server
	// that needs to consume less resources than a regular server.
	Desktop = "Desktop"
)

// AllProfiles Lists all profiles currently available
var AllProfiles = []string{Web, OLTP, DW, Mixed, Desktop}
