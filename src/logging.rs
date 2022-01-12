//! Logging and telemetry module for storing, formatting and displaying events.

use std::{fs::File, sync::Arc};

use tracing::Level;
use tracing_subscriber::{filter::Targets, prelude::*, Registry};

/// Initialize and configure console and file-based logging.
///
/// Provides per-layer and global filtering by leveraging `Targets`. Spans and
/// events are emitted to the console or written to a file depending on their
/// severity.
pub fn initialize_logger() -> std::io::Result<()> {
    // The console layer emits events to stdout and are intended to be human-readable
    let console_logger = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(true)
        .with_filter(Targets::default().with_default(Level::INFO));

    // File logger layer used to store persistent log data;
    let filepath = File::create("stasis.log")?;
    let file_logger = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(Arc::new(filepath));

    let global_targets = vec![("stasis", Level::TRACE), ("tokio", Level::WARN)];
    let global_filter = Targets::default().with_targets(global_targets);

    Registry::default()
        .with(console_logger)
        .with(file_logger)
        .with(global_filter)
        .init();

    Ok(())
}
