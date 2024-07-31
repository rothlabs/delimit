use std::{error::Error, result};

use super::*;

pub trait Solve {
    type Load;
    /// For units to grant a load and NOT act upon external systems
    fn solve(&self) -> Self::Load;
}

pub trait DoSolve {
    type Load;
    /// For graph internals to handle grant calls
    fn do_solve(&mut self, back: &Back) -> Self::Load;
}

pub type Result = result::Result<Tray, Box<dyn Error>>;

pub enum Tray {
    Node(Node)
}