use super::*;
use std::{collections::HashMap, fs::{self, File}, io::BufReader};

#[derive(Default, Clone, Serialize)]
pub struct Repo {
    nodes: HashMap<Id, Node>,
    path: Node,
}

impl Repo {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn path(&mut self, path: impl Into<Node>) -> &mut Self {
        self.path = path.into();
        self
    }
    fn save(&self) -> solve::Result {
        let mut serial = Serial::new();
        for node in self.nodes.values() {
            node.serial(&mut serial)?;
        }
        let path = self.path.string()?;
        let data = serde_json::to_string(&serial)?;
        fs::write(path, data)?;
        Ok(Tray::None)
    }
    fn load(&self) -> alter::Result {
        let path = self.path.string()?;
        // let data = fs::read_to_string(path)?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let serial: Serial = serde_json::from_reader(reader)?;

        Ok(Report::None)
    }
    fn insert(&mut self, nodes: Vec<Node>) -> alter::Result {
        for node in nodes {
            let meta = node.meta();
            self.nodes.insert(meta.id.clone(), node);
        }
        Ok(Report::None)
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
            path: self.path.clone(),
        }
    }
}

impl Alter for Repo {
    fn alter(&mut self, post: Post) -> alter::Result {
        match post.form {
            post::Form::Insert(nodes) => self.insert(nodes),
            post::Form::Cmd(name) => {
                match name.as_str() {
                    LOAD => self.load(),
                    _ => Ok(Report::None)
                }
            },
            _ => Ok(Report::None)
        }
        // if let post::Form::Insert(nodes) = post.form {
        //     self.insert(nodes)
        // }
        // Ok(Report::None)
    }
}

impl Solve for Repo {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            Task::Cmd(name) => {
                match name.as_str() {
                    SAVE => self.save(),
                    _ => Ok(Tray::None)
                }
            }
            _ => Ok(Tray::None)
        }
    }
}

// match name.as_str() {
//     SAVE => self.save()?,
//     LOAD => self.load()?,
//     _ => ()
// }
