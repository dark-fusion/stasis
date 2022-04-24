set dotenv-load := true
set positional-arguments := false

# Set default to display the list of commands
_default:
    @just --list

# Create an optimized 'release' build
@build:
    cargo build --release

# Format, lint and check that project compiles
@compile:
    cargo fmt --all
    cargo clippy -- -D warnings

# Format the project with rustfmt
@format:
    cargo fix
    cargo clippy --fix
    cargo fmt --all

# Quickly format and run linter
@lint:
    cargo fmt --all
    cargo clippy

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test

# Run tests with 'nocapture' and 'quiet' flags set
@test:
    cargo test -- --nocapture --quiet

# Run tests single-threaded for concurrency-related debugging
@test-debug:
    cargo test -- --test-threads=1 --nocapture

