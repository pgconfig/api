GOPATH := $(shell go env GOPATH)
GOBIN := "$(GOPATH)/bin"

mod:
	go mod download
	go install github.com/swaggo/swag/cmd/swag
	go install golang.org/x/lint/golint
	go install github.com/mattn/goveralls@latest

docs: mod clean
	mkdir -p ./cmd/api/docs
	$(GOBIN)/swag init --dir ./cmd/api --output ./cmd/api/docs

test: docs
	go test -race -v -covermode atomic -coverprofile=covprofile ./...

goveralls-push: mod
	$(GOBIN)/goveralls -coverprofile=covprofile -service=github

check: lint

lint: mod
	$(GOBIN)/golint -set_exit_status ./...

clean:
	rm -rf dist/
	rm -rf cmd/api/docs

build: clean docs lint
	mkdir -p dist/
	go build -o dist/pgconfigctl cmd/pgconfigctl/main.go
	go build -o dist/api cmd/api/main.go