use std::collections::HashMap;
use super::*;

pub struct Repo<T> {
    pub parts: HashMap<Meta, T>,
}