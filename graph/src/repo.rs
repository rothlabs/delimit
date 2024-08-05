use super::*;
use std::collections::HashMap;

#[derive(Default, Clone, Serialize)]
pub struct Repo {
    pub nodes: HashMap<Id, Node>,
}

impl Repo {
    pub fn new() -> Self {
        Self::default()
    }
    fn insert(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            let meta = node.meta();
            self.nodes.insert(meta.id.clone(), node);
        }
    }
    fn stems(&self) -> solve::Result {
        let stems = self.nodes.values().cloned().collect();
        Ok(Tray::Nodes(stems))
    }
}

impl Make for Repo {
    fn make(&self, _: &Back) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

impl Alter for Repo {
    fn alter(&mut self, post: Post) -> alter::Result {
        if let post::Form::Insert(nodes) = post.form {
            self.insert(nodes)
        }
        Ok(Report::None)
    }
}

impl Solve for Repo {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            _ => Ok(Tray::None)
        }
    }
}

// match post.form {
//     post::Form::Insert(nodes) => self.insert(nodes),
//     _ => (),
// };
