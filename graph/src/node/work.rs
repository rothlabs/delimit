use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
pub struct Work<T, L> {
    map: HashMap<T, L>,
}