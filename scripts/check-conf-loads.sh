#!/usr/bin/env bash
# Generates a postgresql.conf with pgconfigctl and proves that a real
# PostgreSQL server of that version starts with it and reads every setting
# from it.
#
# Usage: scripts/check-conf-loads.sh <path to pgconfigctl> <postgres version>
set -euo pipefail

bin="${1:?Usage: check-conf-loads.sh <path to pgconfigctl> <postgres version>}"
version="${2:?Usage: check-conf-loads.sh <path to pgconfigctl> <postgres version>}"
container="pgconfig-conf-check-${version}-$$"
workdir="$(mktemp -d)"

cleanup() {
  docker rm -f "$container" >/dev/null 2>&1 || true
  rm -rf "$workdir"
}
trap cleanup EXIT

# The host facts are explicit so the result does not depend on the machine
# that runs the check.
"$bin" tune --version "$version" --format conf \
  --ram 1GB --cpus 2 --os linux --arch amd64 >"$workdir/tuned.conf"
echo "--- Generated config for PostgreSQL $version ---"
cat "$workdir/tuned.conf"

cat >"$workdir/init-config.sh" <<'INIT'
#!/bin/bash
echo "include = '/etc/postgresql/tuned.conf'" >> "$PGDATA/postgresql.conf"
INIT
chmod 755 "$workdir/init-config.sh"
chmod 644 "$workdir/tuned.conf"

docker run -d --name "$container" \
  -e POSTGRES_PASSWORD=postgres \
  -v "$workdir/tuned.conf:/etc/postgresql/tuned.conf:ro" \
  -v "$workdir/init-config.sh:/docker-entrypoint-initdb.d/init-config.sh:ro" \
  "postgres:$version" >/dev/null

# The image starts a temporary server to run the init scripts and then
# restarts. Only the final server has read the include.
ready=0
for _ in $(seq 1 90); do
  if [ -z "$(docker ps -q -f "name=$container")" ]; then
    echo "The container stopped. PostgreSQL $version rejected the config:"
    docker logs "$container"
    exit 1
  fi
  if docker logs "$container" 2>&1 | grep -q "PostgreSQL init process complete" &&
    docker exec "$container" pg_isready -U postgres -h 127.0.0.1 >/dev/null 2>&1; then
    ready=1
    break
  fi
  sleep 1
done
if [ "$ready" -ne 1 ]; then
  echo "Timed out waiting for PostgreSQL $version."
  docker logs "$container"
  exit 1
fi

# Every setting in the file must be in effect from that file. A setting the
# server ignored or took from elsewhere fails the check.
docker exec "$container" psql -U postgres -At -F ' = ' -c \
  "SELECT name, current_setting(name) FROM pg_settings WHERE sourcefile = '/etc/postgresql/tuned.conf' ORDER BY name" \
  >"$workdir/applied.txt"
cat "$workdir/applied.txt"

missing=""
for name in $(grep -oE '^[a-z_]+ = ' "$workdir/tuned.conf" | cut -d' ' -f1); do
  if grep -q "^$name = " "$workdir/applied.txt"; then
    continue
  fi
  # wal_buffers = -1 asks the server to size it. The server then reports its
  # own value and no source file.
  if [ "$name" = "wal_buffers" ] && grep -q '^wal_buffers = -1$' "$workdir/tuned.conf"; then
    continue
  fi
  missing="$missing $name"
done

if [ -n "$missing" ]; then
  echo "PostgreSQL $version did not take these settings from the file:$missing"
  exit 1
fi
echo "PostgreSQL $version loaded every setting in the file."
