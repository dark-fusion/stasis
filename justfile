set dotenv-load := false

# Run code-quality and CI-related tasks locally
pre-commit:
    cargo fmt --all -- --check && \
    cargo clippy -- --D warnings && \
    cargo test && \
    cargo audit

# Run all tests sequentially without capturing IO data such as debug info
test-debug:
    cargo test -- --test-threads=1 --nocapture
