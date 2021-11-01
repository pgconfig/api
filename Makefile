GOPATH := $(shell go env GOPATH)
GOBIN := "$(GOPATH)/bin"

mod:
	go mod download
	go install github.com/swaggo/swag/cmd/swag

docs: mod
	rm -rfv ./cmd/api/docs
	mkdir -p ./cmd/api/docs
	$(GOBIN)/swag init --dir ./cmd/api --output ./cmd/api/docs

test: docs
	go test -v -coverprofile=profile.cov ./...
