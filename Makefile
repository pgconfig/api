GOPATH := $(shell go env GOPATH)
GOBIN := "$(GOPATH)/bin"

mod:
	go install github.com/swaggo/swag/cmd/swag@latest
	go mod download
	go install github.com/mattn/goveralls@latest

docs: mod clean
	mkdir -p ./cmd/api/docs
	$(GOBIN)/swag init --dir ./cmd/api --output ./cmd/api/docs

test: docs
	go test -race -v -covermode atomic -coverpkg=./... -coverprofile=covprofile ./...

goveralls-push: mod
	$(GOBIN)/goveralls -coverprofile=covprofile -service=github -v

check: lint

lint: mod
	go vet ./...

clean:
	rm -rf dist/
	rm -rf cmd/api/docs

build: clean docs lint
	mkdir -p dist/
	go build -o dist/pgconfigctl cmd/pgconfigctl/main.go
	go build -o dist/api cmd/api/main.go

heroku-publish:
	goreleaser