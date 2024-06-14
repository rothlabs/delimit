use std::{ops::{Deref, DerefMut}, sync::{RwLockReadGuard, RwLockWriteGuard}};

use serde::Serialize;

pub struct Read<'a, T> (
    pub RwLockReadGuard<'a, T>,
);

impl<'a, T> Read<'a, T> {
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

impl<'a, T: Serialize> Serialize for Read<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

pub struct Write<'a, T> (
    pub RwLockWriteGuard<'a, T>,
);

impl<'a, T> Write<'a, T> {
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