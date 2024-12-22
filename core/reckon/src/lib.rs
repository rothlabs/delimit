use std::future::Future;
// use std::pin::Pin;

mod apex;
mod node;
mod pass;

type BoxFuture<'a, T> = Box<dyn Future<Output = T> + 'a>; // + Send + 

pub type Key = usize;

pub trait Solve<T> {
    fn solve(&self, key: &Key) -> Result<T>;
}

pub trait Store<T> {
    fn has(&self, key: &Key) -> bool;
    fn set(&mut self, key: &Key, base: T);
    fn get(&self, key: &Key) -> &T;
}

// pub trait Store<T> {
//     fn has(&self, key: Key) -> bool;
//     fn get(&self, key: Key) -> Option<&T>;
//     fn set(&mut self, key: Key, base: T) -> &T;
// }

pub enum Hub<T> {
    Base(T),
    Apex(Key),
}

pub struct Apex<T> {
    unit: T,
    roots: Vec<Key>,
}

struct Pass<'a, G, S> {
    graph: &'a G,
    state: &'a mut S,
    stack: Vec<Key>,
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error(transparent)]
    // Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
    // #[error(transparent)]
    // Generic(#[from] Box<dyn std::error::Error>),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

// struct Dependance {
//     node: Key,
//     stems: Vec<Key>,
// }

// pub struct Node<T, G> {
//     apex: Box<dyn node::Solve<Base = T, Graph = G>>,
//     roots: Vec<Key>,
// }


// pub struct Graph {
//     nodes: Vec<Node>,
//     // a different vec for nodes of different output types
// }

