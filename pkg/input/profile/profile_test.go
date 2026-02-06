package profile

import (
	"testing"

	. "github.com/pgconfig/api/pkg/tests"
)

func TestProfile_Set(t *testing.T) {
	Describe("Profile.Set()", t, func() {
		Context("when input is valid", func() {
			It("should accept WEB in uppercase", func() {
				var p Profile
				err := p.Set("WEB")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Web)
			})

			It("should accept web in lowercase", func() {
				var p Profile
				err := p.Set("web")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Web)
			})

			It("should accept OLTP in uppercase", func() {
				var p Profile
				err := p.Set("OLTP")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, OLTP)
			})

			It("should accept oltp in lowercase", func() {
				var p Profile
				err := p.Set("oltp")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, OLTP)
			})

			It("should accept DW in uppercase", func() {
				var p Profile
				err := p.Set("DW")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, DW)
			})

			It("should accept dw in lowercase", func() {
				var p Profile
				err := p.Set("dw")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, DW)
			})

			It("should accept MIXED in uppercase", func() {
				var p Profile
				err := p.Set("MIXED")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Mixed)
			})

			It("should accept Mixed in mixed case", func() {
				var p Profile
				err := p.Set("Mixed")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Mixed)
			})

			It("should accept mixed in lowercase", func() {
				var p Profile
				err := p.Set("mixed")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Mixed)
			})

			It("should accept DESKTOP in uppercase", func() {
				var p Profile
				err := p.Set("DESKTOP")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Desktop)
			})

			It("should accept Desktop in mixed case", func() {
				var p Profile
				err := p.Set("Desktop")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Desktop)
			})

			It("should accept desktop in lowercase", func() {
				var p Profile
				err := p.Set("desktop")
				Expect(err, ShouldBeNil)
				Expect(p, ShouldEqual, Desktop)
			})
		})

		Context("when input is invalid", func() {
			It("should return an error", func() {
				var p Profile
				err := p.Set("invalid")
				Expect(err, ShouldNotBeNil)
			})
		})
	})
}
