use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct Channel<T: Clone> {
    tx: Sender<T>,
}

impl<T: Clone> Default for Channel<T> {
    fn default() -> Self {
        let (tx, _) = channel::<T>(1);
        Self { tx }
    }
}

impl<T: Clone> Channel<T> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = channel::<T>(capacity);
        Self { tx }
    }
}

impl<T: Clone> Clone for Channel<T> {
    fn clone(&self) -> Self {
        Self { tx: self.tx.clone() }
    }
}

impl<T: Clone> Channel<T> {
    pub fn rx(&self) -> Receiver<T> {
        self.tx.subscribe()
    }

    pub fn tx(&self) -> Sender<T> {
        self.tx.clone()
    }
}
