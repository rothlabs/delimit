use std::sync::{Weak, RwLock};

pub struct Back<B> {
    body: Weak<RwLock<B>>,
}
