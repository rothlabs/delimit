use std::sync::{Arc, RwLock};

use crate::*;

#[derive(Clone)]
pub struct Solver<U, W>{
    edge: Arc<RwLock<edge::Solver<U, W>>>,
    meta: Meta,
}