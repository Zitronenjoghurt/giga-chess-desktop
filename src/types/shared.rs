use crate::persistence::PersistentObject;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Shared<T> {
    pub inner: Arc<Mutex<T>>,
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.inner.lock().unwrap()
    }

    pub fn with<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock();
        f(&mut *guard)
    }

    pub fn set(&self, value: T) {
        *self.lock() = value;
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Default> Default for Shared<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone> Shared<T> {
    pub fn get_clone(&self) -> T {
        self.lock().clone()
    }
}

impl<T: Copy> Shared<T> {
    pub fn get_copy(&self) -> T {
        *self.lock()
    }
}

impl<T: PersistentObject> PersistentObject for Shared<T> {
    type PersistentType = T::PersistentType;

    fn save_state(&self) -> Self::PersistentType {
        self.lock().save_state()
    }

    fn load_from_state(state: Self::PersistentType) -> Self {
        Self::new(T::load_from_state(state))
    }
}
