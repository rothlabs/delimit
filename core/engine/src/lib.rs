// use std::cell::Cell;
use std::pin::Pin;
// use std::sync::Arc;
// use std::rc::{Rc, self};
use std::future::Future;

pub mod store;
pub mod agent;

pub type Grc<T> = std::rc::Rc<T>;
type Weak<T> = std::rc::Weak<T>;
type Cell<T> = std::cell::RefCell<T>;
type PinFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct Store<T> {
    pub len: usize,
    free_head: usize,
    data: Vec<store::Entry<T>>,
}

pub struct Agent<T> {
    pub unit: T,
    // out:
}

pub enum Hub<T> {
    Base(T),
    // Agent
}

pub type Result<T> = std::result::Result<T, Error>;

/// Graph Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}
