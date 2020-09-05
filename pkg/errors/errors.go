package errors

import "errors"

var (
	ErrorInvalidSchema       = errors.New("Invalid schema")
	ErrorInputMaxConnections = errors.New("Please set a value > 0")
	ErrorInvalidOS           = errors.New("Invalid OS")
	ErrorInvalidArch         = errors.New("Invalid Architecture")
)
