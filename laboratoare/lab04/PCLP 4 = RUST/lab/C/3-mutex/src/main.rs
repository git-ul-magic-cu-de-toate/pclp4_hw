use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

struct Mutex<T> {
    cell: UnsafeCell<T>,
    state: AtomicU32,
}

struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Mutex {
            cell: UnsafeCell::new(value),
            state: AtomicU32::new(0),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        // Set the state to 1: locked.
        while self.state.swap(1, Ordering::Acquire) == 1 {
            // If it was already locked, wait.
            wait(&self.state, 1);
        }
        MutexGuard { mutex: self }
    }

    fn unlock(&self) {
        // Set the state to 0: unlocked.
        self.state.store(0, Ordering::Release);
        // Notify one waiting thread, if any.
        wake_one(&self.state);
    }

    pub fn into_inner(self) -> T {
        // Safety: Since we consume the Mutex, there is no need to lock.
        UnsafeCell::into_inner(self.cell)
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: we have a shared reference to the mutex guard,
        // and therefore have (shared) access to the value protected by the mutex
        unsafe { &*self.mutex.cell.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: we have an exclusive reference to the mutex guard,
        // and therefore have exclusive access to the value protected by the mutex
        unsafe { &mut *self.mutex.cell.get() }
    }
}

impl<_, T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

fn wait(state: &AtomicU32, value: u32) {
