use super::*;

/// Mesh validation. Produces new mesh with go/no-go attribute. 
/// "Go" means the triangle intersections with exactly three trianlges along their edges.
/// "No-go" means the triangle intersections in some other way such as a crossing.
#[derive(Default, Clone, Debug)]
pub struct Validator {
    /// Expect non-indexed single-attribute of XYZ position.
    vector: Node,
    /// Distance threshold for hit/miss
    tolerance: Node,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vector(&mut self, vector: impl Into<Node>) -> &mut Self {
        self.vector = vector.into();
        self
    } 
    pub fn tolerance(&mut self, tolerance: impl Into<Node>) -> &mut Self {
        self.tolerance = tolerance.into();
        self
    } 
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.vector = self.vector.trade(trade);
        self.tolerance = self.tolerance.trade(trade);
        Ok(Gain::None)
    }
    fn main(&self) -> solve::Result {
        
        Ok(Tray::None)
    }
}

impl Adapt for Validator {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Validator {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            _ => did_not_solve(),
        }
    }
}

