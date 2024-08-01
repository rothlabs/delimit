use std::result;

use super::*;

pub trait Solve {
    /// Solve a task. 
    /// The node will perform computations or return an existing result.
    fn solve(&self) -> Result;
}

pub trait DoSolve {
    /// For graph internals to handle solve calls
    fn do_solve(&mut self, back: &Back) -> Result;
}

pub type Result = result::Result<Tray, Error>;

#[derive(Clone, PartialEq)]
pub enum Tray {
    Node(Node),
    //Load(Load),
    None
}

impl From<Node> for Tray {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

pub trait IntoTray {
    fn tray(self) -> Tray;
}

impl<T> IntoTray for T 
where 
    T: Into<Tray>
{
    fn tray(self) -> Tray {
        self.into()
    }
}

pub struct Query<T> {
    item: T,
}

impl<T> Query<T> 
where 
    T: Solve + Clone
{
    pub fn new(item: &T) -> Self {
        Self { item: item.clone() }
    }
    pub fn node(&self) -> node::Result {
        match self.item.solve()? {
            Tray::Node(node) => Ok(node),
            _ => Err("not a node".into())
        } 
    }
    // pub fn load(&self) -> load::Result {
    //     match self.item.solve()? {
    //         Tray::Load(load) => Ok(load),
    //         _ => Err("not a load".into())
    //     } 
    // }
}

pub trait ToQuery<T> {
    fn query(&self) -> Query<T>;
}

impl<T> ToQuery<T> for T 
where 
    T: Solve + Clone
{
    fn query(&self) -> Query<T> {
        Query { item: self.clone() }
    }    
}