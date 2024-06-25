use std::{collections::HashMap, hash::Hash};

pub struct Work<T, L> {
    map: HashMap<T, L>,
}