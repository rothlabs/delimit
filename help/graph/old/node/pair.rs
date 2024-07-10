use std::marker::PhantomData;

use crate::*;

pub struct Pair<U, S> {
    pub unit: U,
    stem: PhantomData<S>,
}

impl<U, S> FromUnit for Pair<U, S> {
    type Unit = U;
    fn from_unit(unit: Self::Unit) -> Self {
        Self {
            unit,
            stem: PhantomData {},
        }
    }
}

impl<U, S> Read for Pair<U, S> {
    type Unit = U;
    fn read(&self) -> &U {
        &self.unit
    }
}
