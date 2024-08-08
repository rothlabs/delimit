use regex::Regex;

use super::*;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Bay {
    /// The nodes can be another bay
    nodes: HashMap<Path, Node>,
    path: Node,
    dump: String,
    #[serde(skip)]
    deserializer: Option<Box<dyn DeserializeNode>>,
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn path(&mut self, path: impl Into<Node>) -> &mut Self {
        self.path = path.into();
        self
    }
    pub fn deserializer(&mut self, deserial: Box<dyn DeserializeNode>) -> &mut Self {
        self.deserializer = Some(deserial);
        self
    }
    fn extend(&mut self, nodes: Vec<Node>) -> adapt::Result {
        for node in nodes {
            let meta = node.meta();
            self.nodes.insert(meta.path.clone(), node);
        }
        Ok(Gain::None)
    }
    fn stems(&self) -> solve::Result {
        let stems = self.nodes.values().cloned().collect();
        Ok(Tray::Nodes(stems))
    }
    fn export(&self) -> solve::Result {
        let mut serial = Serial::new();
        for node in self.nodes.values() {
            node.serial(&mut serial)?;
        }
        let path = self.path.string()?;
        let data = serde_json::to_string(&serial)?;
        fs::write(path, data)?;
        Ok(Tray::None)
    }
    fn import(&mut self) -> adapt::Result {
        let deserializer = self.deserializer.as_ref().ok_or("missing deserializer")?;
        let path = self.path.string()?;
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let serial: Serial = serde_json::from_reader(reader)?;
        self.dump = String::new();
        for (id, part) in &serial.parts {
            if let Ok(node) = deserializer.deserialize(part) {
                self.nodes.insert(id.into(), node);
                self.dump += &(part.to_owned() + "\n" + "gnid==" + id + "\n");
            } else {
                panic!("failed to load part: {}", part)
            }
        }
        fs::write("/home/julian/delimit/repo/storage/debug.txt", &self.dump)?;
        Ok(Gain::None)
    }
    fn find(&self, regex: &str) -> solve::Result {
        let re = Regex::new(regex)?; //Regex::new(r"(?P<story>Delimit index page)")?;
        let caps = re.captures(&self.dump).ok_or("no match")?;
        let start = caps.get(0).unwrap().start();
        let caps = Regex::new("gnid==([a-zA-Z0-9]{16})")?
            .captures_at(&self.dump, start)
            .ok_or("no match")?;
        let id = caps.get(1).unwrap().as_str();
        let node = self.nodes.get(id).ok_or("id not found")?.clone();
        Ok(Tray::Node(node))
    }
}

impl Adapt for Bay {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(_) => Ok(Gain::None),
            Post::Extend(nodes) => self.extend(nodes),
            Post::Import => self.import(),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            Task::Export => self.export(),
            Task::Find(regex) => self.find(&regex),
            _ => did_not_solve(),
        }
    }
}

// impl Make for Repo {
//     fn make(&self, _: &Back) -> Self {
//         Self {
//             nodes: self.nodes.clone(),
//             path: self.path.clone(),
//             pool: self.pool.clone(),
//             deserializer: self.deserializer.clone(),
//         }
//     }
// }

// Task::Cmd(name) => match name.as_str() {
//     SAVE => self.save(),
//     _ => Ok(Tray::None),
// },
// post::Form::Cmd(name) => match name.as_str() {
//     LOAD => self.load(),
//     _ => Ok(Report::None),
// },

// let mut debug = String::new();
// if let Ok(node) = node_result {
//     self.nodes.insert(id.into(), node);
//     all.push_str(string);
//     all.push_str("\n\n");
// }
// // if let Err(_) = node_result {
// //     all.push_str(&string);
// //     all.push_str(&"\n\n");
// // }

// debug.push_str(&self.nodes.len().to_string());
//         fs::write("/home/julian/delimit/repo/storage/debug.txt", debug)?;
