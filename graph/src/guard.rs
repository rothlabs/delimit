use std::{ops::{Deref, DerefMut}, sync::{RwLockReadGuard, RwLockWriteGuard}};

pub struct Read<'a, T: ?Sized> (
    pub RwLockReadGuard<'a, T>,
);

impl<'a, T: ?Sized> Read<'a, T> {
    pub fn new(guard: RwLockReadGuard<'a, T>) -> Self {
        Self(guard)
    }
}

impl<T> Deref for Read<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Write<'a, T: ?Sized> (
    pub RwLockWriteGuard<'a, T>,
);

impl<'a, T: ?Sized> Write<'a, T> {
    pub fn new(guard: RwLockWriteGuard<'a, T>) -> Self {
        Self(guard)
    }
}

impl<T> Deref for Write<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Write<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}