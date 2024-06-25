use std::marker::PhantomData;

use crate::*;


pub struct Pointer<U, S> {
    pub unit: U,
    stem: PhantomData<S>,
}

impl<U, S> FromUnit for Pointer<U, S> {
    type Unit = U;
    fn from_unit(unit: Self::Unit) -> Self {
        Self {
            unit,
            stem: PhantomData{},
        }
    }
}

impl<U, S> Read for Pointer<U, S> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}

