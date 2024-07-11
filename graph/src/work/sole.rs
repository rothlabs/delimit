use serde::Serialize;

use crate::*;

/// Wrapper around a single Load as opposed to Work that must solve for a Load
#[derive(Serialize)]
pub struct Sole<L> {
    load: L,
}

impl<L> FromItem for Sole<L> {
    type Item = L;
    fn new(load: Self::Item) -> Self {
        Self { load }
    }
}

impl<L> ToLoad for Sole<L>
where
    L: Clone,
{
    type Load = L;
    fn load(&self) -> Self::Load {
        self.load.clone()
    }
}

impl<L> Read for Sole<L> {
    type Unit = L;
    fn read(&self) -> &Self::Unit {
        &self.load
    }
}

impl<L> Write for Sole<L> {
    type Unit = L;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F) {
        write(&mut self.load);
    }
}
