use super::Store;

/// A wrapper around a `Store` instance. This exists to allow orderly cleanup
/// of the `Store` by signaling the background purge task to shut down when
/// this struct is dropped.
#[derive(Debug, Default)]
pub struct Engine {
    /// The `Store` instance shut down when this `Engine` instance is dropped.
    store: Store,
}

impl Engine {
    /// Create a new `Engine`, wrapping a `Store` instance.
    pub fn new() -> Engine {
        Engine {
            store: Store::new(),
        }
    }

    /// Get the shared database.
    pub fn store(&self) -> Store {
        self.store.clone()
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.store.shutdown_purge_task();
    }
}
