# The server embeds web/dist, so the web app is built before cargo is.

# Build the web app.
web:
    cd web && npm ci && npm run build

# Run every test: the web app, then the workspace, goldens included.
test: web
    cd web && npm test
    cargo test

# Check formatting and lints.
lint:
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings

# Serve the API, the web app, and MCP on http://localhost:3000.
run: web
    cargo run -p pgconfig-server

# Load a generated config in a real PostgreSQL. Needs Docker.
check-conf version="18" image="":
    cargo build -p pgconfigctl
    scripts/check-conf-loads.sh target/debug/pgconfigctl {{version}} "{{image}}"

# Windows compiles only on its own runner, so it is left out. The builds run
# one at a time: two `cargo zigbuild` runs that start together race on their
# linker wrappers.

# Rehearse the release build of the Linux and macOS targets.
release-build: web
    goreleaser build --snapshot --clean --parallelism 1 \
        --id pgconfigctl-linux --id pgconfig-server-linux \
        --id pgconfigctl-macos --id pgconfig-server-macos
