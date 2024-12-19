use std::future::Future;
// use std::pin::Pin;

mod node;

type BoxFuture<'a, T> = Box<dyn Future<Output = T> + 'a>; // + Send + 

pub type Key = usize;

pub trait Solve<T> {
    fn solve(&mut self, hub: &Hub<T>) -> &T;
}

pub enum Hub<T> {
    Base(T),
    Node(Key),
}

pub struct Node<T, G> {
    apex: Box<dyn node::Solve<Base = T, Graph = G>>,
    roots: Vec<Key>,
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


// pub struct Graph {
//     nodes: Vec<Node>,
//     // a different vec for nodes of different output types
// }

