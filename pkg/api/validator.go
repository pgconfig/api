package api

import (
	"github.com/pgconfig/api/pkg/config"
	"github.com/pgconfig/api/pkg/errors"
	"github.com/pgconfig/api/pkg/rules"
)

type validator struct {
	Errors []string
}

func newValidator() *validator {
	return &validator{}
}

func (v *validator) validInputs(body config.Input) {
	var err error
	if body.MaxConnections < 1 {
		v.Errors = append(v.Errors, errors.ErrorInputMaxConnections.Error())
	}
	if err = rules.ValidOS(body.OS); err != nil {
		v.Errors = append(v.Errors, err.Error())
	}
	if err = rules.ValidArch(body.Arch); err != nil {
		v.Errors = append(v.Errors, err.Error())
	}
}

func (v *validator) hasErrors() bool {
	return len(v.Errors) > 0
}
