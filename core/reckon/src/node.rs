use super::*;

// maybe use anyhow::Error instead of Box<dyn Error>
// pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait Solve {
    type Base;
    type Graph;
    fn solve(&self, graph: &Self::Graph) -> impl Future<Output = Result<Self::Base>>;
}