use super::*;
use std::collections::HashMap;

pub struct Repo<T> {
    pub parts: HashMap<Meta, Node<T>>,
}


