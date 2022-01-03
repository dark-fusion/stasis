set dotenv-load := false

# Run CI tasks locally
check:
    cargo fmt --all -- --check && \
    cargo clippy -- --D warnings && \
    cargo test && \
    cargo audit
