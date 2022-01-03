use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(t);
        // Manually drop the `queue`
        drop(queue);
        self.inner.cvar.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Sender {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}

impl<T> Receiver<T> {
    /// Receive a message from the sender
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();

        // Loop until a signal is received by the `Condvar` to wake up thread
        loop {
            match queue.pop_front() {
                Some(t) => return t,
                None => {
                    queue = self.inner.cvar.wait(queue).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    cvar: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
        cvar: Condvar::new(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
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
        assert_eq!(rx.recv(), 42);
    }
}
