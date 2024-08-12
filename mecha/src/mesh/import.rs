use regex::Regex;
use super::*;
use std::{
    fs::{self, File},
    io::BufReader,
};

/// Mesh import.
#[derive(Default, Clone, Debug)]
pub struct Import {
    /// Expect file system path String
    path: Node,
}

impl Import {
    pub fn new() -> Self {
        Self::default()
    }

    /// Trade nodes for others with same semantics and different graph properties
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.path = self.path.trade(trade);
        Ok(Gain::None)
    }

    /// Importer main function
    fn main(&self) -> solve::Result {
        let path = self.path.string()?;
        let data = fs::read_to_string(path)?;
        let re = Regex::new(r"vertex (-?[0-9]\d*(\.\d+)?) (-?[0-9]\d*(\.\d+)?) (-?[0-9]\d*(\.\d+)?)")?;
        let mut mesh = vec![];
        for (_, [x, y, z]) in re.captures_iter(&data).map(|c| c.extract()) {
            mesh.push(x.parse::<f64>()?);
            mesh.push(y.parse::<f64>()?);
            mesh.push(z.parse::<f64>()?);
        }
        Ok(mesh.leaf().node().tray())
    }
}

impl Adapt for Import {
    /// Mutate node by Post options
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Import {
    /// Solve node by Task options
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            _ => did_not_solve(),
        }
    }
}