use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU8, Ordering};

const INIT: u8 = 0;
const ACQUIRED: u8 = 1;

struct Mutex<T> {
    value: UnsafeCell<T>,
    status: AtomicU8,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            status: AtomicU8::new(INIT),
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .status
            .compare_exchange(INIT, ACQUIRED, Ordering::Acquire, Ordering::Relaxed)
        {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.status.store(INIT, Ordering::Release);
    }
}