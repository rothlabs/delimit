use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use serde::Serialize;

use super::{React, Solve, UnitRef};

#[derive(Clone, Serialize)]
pub struct Node<U, T, L, V> {
    pub unit: U,
    pub work: HashMap<T, L>,
    vary: PhantomData<V>,
}

impl<U, T, L, V> React for Node<U, T, L, V>
where
    U: React<Vary = V>,
{
    type Vary = V;
    fn react(&mut self, vary: V) {
        self.unit.react(vary);
    }
}
