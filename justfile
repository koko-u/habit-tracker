ws_root := justfile_directory()
backend_dir := ws_root / "backend"
backend_manifest := backend_dir / "Cargo.toml"
compose_file := ws_root / "docker/compose.yml"

# Show available recipes
_default:
    @just --list

# Run application
run:
    cargo run --manifest-path "{{backend_manifest}}"

# Watch application
watch:
    cargo watch --workdir "{{backend_dir}}" --exec run

# Format code
fmt:
    cargo +nightly fmt --manifest-path "{{backend_manifest}}"

# Start PostgreSQL and pgAdmin
dup:
    docker compose --file "{{compose_file}}" up --detach

# Stop PostgreSQL and pgAdmin
dwn:
    docker compose --file "{{compose_file}}" down
