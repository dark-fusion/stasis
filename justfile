set dotenv-load := false

# Run CI tasks locally
ci:
    cargo fmt --all -- --check && \
    cargo clippy -- --D warnings && \
    cargo test && \
    cargo audit
