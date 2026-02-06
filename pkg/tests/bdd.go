package tests

import (
	"testing"

	"github.com/smartystreets/goconvey/convey"
)

// Describe is the top-level container for a group of tests
// Maps to RSpec's "describe"
func Describe(description string, t *testing.T, fn func()) {
	convey.Convey(description, t, fn)
}

// Context describes a specific context or scenario within a test
// Maps to RSpec's "context"
func Context(description string, fn func()) {
	convey.Convey(description, fn)
}

// It describes a specific behavior or expectation
// Maps to RSpec's "it"
func It(description string, fn func()) {
	convey.Convey(description, fn)
}

// Expect is an alias for So() to make assertions more readable
// Usage: Expect(value, ShouldEqual, expected)
func Expect(actual any, assertion convey.Assertion, expected ...any) {
	convey.So(actual, assertion, expected...)
}

// Re-export common matchers from GoConvey
var (
	// Equality matchers
	ShouldEqual          = convey.ShouldEqual
	ShouldNotEqual       = convey.ShouldNotEqual
	ShouldResemble       = convey.ShouldResemble
	ShouldNotResemble    = convey.ShouldNotResemble
	ShouldPointTo        = convey.ShouldPointTo
	ShouldNotPointTo     = convey.ShouldNotPointTo
	ShouldBeNil          = convey.ShouldBeNil
	ShouldNotBeNil       = convey.ShouldNotBeNil
	ShouldBeTrue         = convey.ShouldBeTrue
	ShouldBeFalse        = convey.ShouldBeFalse
	ShouldBeZeroValue    = convey.ShouldBeZeroValue

	// Numeric matchers
	ShouldBeGreaterThan          = convey.ShouldBeGreaterThan
	ShouldBeGreaterThanOrEqualTo = convey.ShouldBeGreaterThanOrEqualTo
	ShouldBeLessThan             = convey.ShouldBeLessThan
	ShouldBeLessThanOrEqualTo    = convey.ShouldBeLessThanOrEqualTo
	ShouldBeBetween              = convey.ShouldBeBetween
	ShouldNotBeBetween           = convey.ShouldNotBeBetween

	// Collection matchers
	ShouldContain         = convey.ShouldContain
	ShouldNotContain      = convey.ShouldNotContain
	ShouldBeIn            = convey.ShouldBeIn
	ShouldNotBeIn         = convey.ShouldNotBeIn
	ShouldBeEmpty         = convey.ShouldBeEmpty
	ShouldNotBeEmpty      = convey.ShouldNotBeEmpty
	ShouldHaveLength      = convey.ShouldHaveLength

	// String matchers
	ShouldStartWith        = convey.ShouldStartWith
	ShouldNotStartWith     = convey.ShouldNotStartWith
	ShouldEndWith          = convey.ShouldEndWith
	ShouldNotEndWith       = convey.ShouldNotEndWith
	ShouldBeBlank          = convey.ShouldBeBlank
	ShouldNotBeBlank       = convey.ShouldNotBeBlank
	ShouldContainSubstring = convey.ShouldContainSubstring

	// Panic matchers
	ShouldPanic        = convey.ShouldPanic
	ShouldNotPanic     = convey.ShouldNotPanic
	ShouldPanicWith    = convey.ShouldPanicWith
	ShouldNotPanicWith = convey.ShouldNotPanicWith

	// Error matchers
	ShouldBeError = convey.ShouldBeError

	// Type matchers
	ShouldHaveSameTypeAs    = convey.ShouldHaveSameTypeAs
	ShouldNotHaveSameTypeAs = convey.ShouldNotHaveSameTypeAs
	ShouldImplement         = convey.ShouldImplement
	ShouldNotImplement      = convey.ShouldNotImplement
)
