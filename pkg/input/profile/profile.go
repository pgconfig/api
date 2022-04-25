package profile

import (
	"fmt"
	"strings"
)

// Profile defines the Profile type
type Profile string

const (

	// Web profile
	Web Profile = "WEB"

	// OLTP profile
	OLTP Profile = "OLTP"

	// DW profile
	DW Profile = "DW"

	// Mixed profile
	Mixed Profile = "Mixed"

	// Desktop is the development machine on any non-production server
	// that needs to consume less resources than a regular server.
	Desktop Profile = "Desktop"
)

// AllProfiles Lists all profiles currently available
var AllProfiles = []Profile{Web, OLTP, DW, Mixed, Desktop}

// String is used both by fmt.Print and by Cobra in help text
func (e *Profile) String() string {
	return string(*e)
}

// Set must have pointer receiver so it doesn't change the value of a copy
func (e *Profile) Set(v string) error {

	newV := Profile(strings.ToUpper(v))

	switch newV {
	case Web, OLTP, DW, Mixed, Desktop:
		*e = newV
		return nil
	default:
		return fmt.Errorf("must be one of %v", AllProfiles)
	}
}

// Type is only used in help text
func (e *Profile) Type() string {
	return "Profile"
}
