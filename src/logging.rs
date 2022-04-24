//! Logging and telemetry module for storing, formatting and displaying events.

use std::env;
use std::fs::File;
use std::io::{self, stderr, stdout};
use std::path::Path;
use std::sync::Arc;

use tracing::Level;
use tracing_subscriber::{prelude::*, EnvFilter, Registry};

/// Initializes a logger that records span and event data, emitting structured
/// logs. The level filter and/or directives are set by the RUST_LOG environment
/// variable.
pub fn initialize_logger() -> io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "stasis=debug,tokio=info");
    }

    // Create console logger to emit messages to stdout / stderr
    let console_logger = tracing_subscriber::fmt::layer().with_writer(
        // Traces with `WARN` or `ERROR` are sent to stderr
        stderr
            .with_max_level(Level::WARN)
            // All other traces are sent to stdout
            .or_else(stdout),
    );

    // Create a debug log that will capture all traces
    let debug_log = create_debug_log(Path::new("stasis.json"))?;
    let file_logger = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(debug_log));

    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(console_logger)
        .with(file_logger)
        .init();

    Ok(())
}

// File logger layer used to store persistent log data;
fn create_debug_log(path: &Path) -> io::Result<File> {
    let target_dir = std::env::current_dir()?.join("logs");

    if !target_dir.exists() {
        std::fs::create_dir(&target_dir)?;
    }

    File::options()
        .append(true)
        .read(true)
        .create(true)
        .open(target_dir.join(path))
}
