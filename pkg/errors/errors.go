package errors

import "errors"

var (
	// ErrorInvalidSchema error
	ErrorInvalidSchema = errors.New("Invalid schema")

	// ErrorInputMaxConnections error
	ErrorInputMaxConnections = errors.New("Please set a value > 0")

	// ErrorInvalidOS error
	ErrorInvalidOS = errors.New("Invalid OS")

	// ErrorInvalidArch error
	ErrorInvalidArch = errors.New("Invalid Architecture")
)
