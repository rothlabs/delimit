use std::{ops::Deref, sync::RwLockReadGuard};

pub struct Guard<'a, T: ?Sized> (
    pub RwLockReadGuard<'a, T>,
);

impl<'a, T: ?Sized> Guard<'a, T> {
    pub fn new(rw_lock_guard: RwLockReadGuard<'a, T>) -> Self {
        Guard (
            rw_lock_guard,
        )
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}