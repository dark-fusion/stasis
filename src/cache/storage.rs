use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use bytes::{Bytes, BytesMut};
use parking_lot::Mutex;
use tokio::sync::Notify;
use tokio::time::{self, Duration, Instant};

/// Server state shared across all connections.
///
/// `Cache` contains a `HashMap` storing the key/value data and all
/// `broadcast::Sender` values for active pub/sub channels.
///
/// A `Cache` instance is a handle to shared state. Cloning `Cache` is shallow and
/// only incurs an atomic ref count increment.
///
/// When a `Cache` value is created, a background task is spawned. This task is
/// used to expire values after the requested duration has elapsed. The task
/// runs until all instances of `Cache` are dropped, at which point the task
/// terminates.
#[derive(Clone, Debug)]
pub struct Store {
    /// Handle to the inner `Shared` state, wrapped with an Arc
    shared: Arc<Shared>,
}

impl Default for Store {
    fn default() -> Self {
        Store::new()
    }
}

impl Store {
    /// Create a new `Store` instance. Allocates shared state and spawns a
    /// background task to manage key expiration.
    pub fn new() -> Store {
        let shared = Arc::new(Shared::new(State::new(), Notify::new()));

        // Start the background task.
        tokio::spawn(shared.clone().run_eviction_notifier());

        Store { shared }
    }

    /// Get the value associated with a key or returns `None` if the key does
    /// not exist.
    ///
    /// The key may not exist if it was evicted due to expiration.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock();
        state.entries.get(key).cloned().map(|entry| entry.data)
    }

    /// Inserts an entry into the cache with an optional duration (in seconds)
    pub fn set(&self, key: String, value: Bytes, expiration: Option<Duration>) {
        let mut state = self.shared.state.lock();

        let id = state.next_id;
        state.next_id += 1;

        let mut notify = false;
        let expires_at = expiration.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|expiration| expiration > when)
                .unwrap_or(true);

            state.expirations.insert((when, id), key.clone());
            when
        });

        let prev = state.entries.insert(
            key,
            Entry {
                id,
                data: value,
                expires_at,
            },
        );

        // If there was a value previously associated with the key **and** it
        // had an expiration time. The associated entry in the `expirations` map
        // must also be removed. This avoids leaking data.
        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                state.expirations.remove(&(when, prev.id));
            }
        }

        // Release the mutex before notifying the background task. This helps
        // reduce contention by avoiding the background task waking up only to
        // be unable to acquire the mutex due to this function still holding it.
        drop(state);

        if notify {
            self.shared.eviction_task.notify_one();
        }
    }

    /// Parses a payload received from a client into a database command.
    ///
    /// TODO: Handle parsing of command (the data received from a client)
    pub fn parse_command(self: Arc<Self>, _data: BytesMut) -> std::io::Result<Bytes> {
        Ok(Bytes::new())
    }

    pub(crate) fn shutdown_purge_task(&self) {
        let mut state = self.shared.state.lock();
        state.shutdown = true;

        // Drop the guard prior to signaling the waker.
        drop(state);

        // Notify the waker to wake up the eviction background task
        self.shared.eviction_task.notify_one();
    }
}

#[derive(Debug)]
struct Shared {
    // The shared application state.
    state: Mutex<State>,

    // Background task for handling cache entry evictions.
    eviction_task: Notify,
}

impl Shared {
    pub fn new(state: State, eviction_task: Notify) -> Self {
        Self {
            state: Mutex::new(state),
            eviction_task,
        }
    }

    /// Purge all expired keys and return the `Instant` at which the **next**
    /// key will expire. The background task will sleep until this instant.
    fn evict_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock();

        if state.shutdown {
            // The database is shutting down. All handles to the shared state
            // have dropped. The background task should exit.
            return None;
        }

        // This is needed to make the borrow checker happy. In short, `lock()`
        // returns a `MutexGuard` and not a `&mut State`. The borrow checker is
        // not able to see "through" the mutex guard and determine that it is
        // safe to access both `state.expirations` and `state.entries` mutably,
        // so we get a "real" mutable reference to `State` outside the loop.
        let state = &mut *state;

        // Find all keys scheduled to expire **before** now.
        let now = Instant::now();

        while let Some((&(when, id), key)) = state.expirations.iter().next() {
            if when > now {
                // Done purging, `when` is the instant at which the next key
                // expires. The worker task will wait until this instant.
                return Some(when);
            }

            // The key expired, remove it
            state.entries.remove(key);
            state.expirations.remove(&(when, id));
        }

        None
    }

    async fn run_eviction_notifier(self: Arc<Shared>) {
        while !self.is_shutdown() {
            if let Some(when) = self.evict_expired_keys() {
                tokio::select! {
                    _ = time::sleep_until(when) => {}
                    _ = self.eviction_task.notified() => {}
                }
            } else {
                self.eviction_task.notified().await;
            }
        }

        println!("Purge background task shut down")
    }

    /// Returns `true` if the database is shutting down
    ///
    /// The `shutdown` flag is set when all `Db` values have dropped, indicating
    /// that the shared state can no longer be accessed.
    fn is_shutdown(&self) -> bool {
        self.state.lock().shutdown
    }
}

/// Optimize: Replace concurrent data structures within the `State` struct
/// The internal `State` of the cache, holding the entries and expiration data.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct State {
    /// The main key-value data. This holds all keys and associated values in
    /// the cache. As such, this should use a highly-optimized data structure.
    entries: HashMap<String, Entry>,
    /// Track the TTLs for cache entries.
    expirations: BTreeMap<(Instant, u64), String>,
    /// Unique identifier to use for the next expiring cache entry.
    next_id: u64,
    /// When set to `true`, the shutdown signal is sent to the background task.
    shutdown: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            expirations: BTreeMap::new(),
            next_id: 0,
            shutdown: false,
        }
    }

    pub fn next_expiration(&self) -> Option<Instant> {
        self.expirations
            .keys()
            .next()
            .map(|expiration| expiration.0)
    }
}

/// Entry in the key-value store
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Entry {
    /// Uniquely identifies this entry.
    pub id: u64,
    /// Stored data as `Bytes`.
    pub data: Bytes,
    /// Instant that determines when the entry should be removed from the cache.
    pub expires_at: Option<Instant>,
}
