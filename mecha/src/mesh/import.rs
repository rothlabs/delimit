use super::*;
use regex::{Captures, Regex};
use std::fs;

/// STL mesh import.
#[derive(Default, Clone, Debug)]
pub struct Import {
    /// Expect file system path String of stl file.
    path: Node,
}

impl Import {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set import path
    pub fn path(&mut self, path: impl Into<Node>) -> &mut Self {
        self.path = path.into();
        self
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
        let re = Regex::new(r"vertex (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+)")?;
        let mut mesh = vec![];
        for caps in re.captures_iter(&data) {
            mesh.push(self.component(&caps, 1)?);
            mesh.push(self.component(&caps, 2)?);
            mesh.push(self.component(&caps, 3)?);
        }
        Ok(mesh.leaf().node().tray())
    }

    /// X, Y, or Z component from capture groups
    fn component(&self, caps: &Captures, group_index: usize) -> Result<f64, Error> {
        Ok(caps
            .get(group_index)
            .ok_or("import failed")?
            .as_str()
            .parse::<f64>()?)
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

// mesh.push(
//     caps.get(1)
//         .ok_or("import failed")?
//         .as_str()
//         .parse::<f64>()?,
// );
// mesh.push(
//     caps.get(2)
//         .ok_or("import failed")?
//         .as_str()
//         .parse::<f64>()?,
// );
// mesh.push(
//     caps.get(3)
//         .ok_or("import failed")?
//         .as_str()
//         .parse::<f64>()?,
// );
