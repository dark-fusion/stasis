use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

// TODO: Sender failure if `send` called with disconnected Receiver

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    /// Push a message onto the channel
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.cvar.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.cvar.notify_one();
        }
    }
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Receiver<T> {
    /// Receive a message from the channel
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();

        // Loop until a signal is received by the `Condvar` to wake up thread
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.cvar.wait(inner).unwrap();
                }
            };
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    cvar: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };

    let shared = Shared {
        inner: Mutex::new(inner),
        cvar: Condvar::new(),
    };

    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(42));
    }

    #[test]
    fn sender_disconnected() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn receiver_disconnected() {
        let (mut tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
