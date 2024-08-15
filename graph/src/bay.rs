// use regex::Regex;

use super::*;
use std::{
    collections::HashMap,
    // fs::{self, File},
    // io::BufReader,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Bay {
    /// The apexes can be another bay
    apexes: HashMap<Key, Apex>,
    path: Apex,
    dump: String,
    #[serde(skip)]
    deserializer: Option<Box<dyn DeserializeApex>>,
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn path(&mut self, path: impl Into<Apex>) -> &mut Self {
        self.path = path.into();
        self
    }
    pub fn deserializer(&mut self, deserial: Box<dyn DeserializeApex>) -> &mut Self {
        self.deserializer = Some(deserial);
        self
    }
    fn extend(&mut self, _: Vec<Apex>) -> adapt::Result {
        // for apex in apexes {
        //     let meta = apex.path();
        //     self.apexes.insert(meta.keys[0].clone(), apex);
        // }
        adapt_ok()
    }
    fn stems(&self) -> solve::Result {
        let stems = self.apexes.values().cloned().collect();
        Ok(Gain::Apexes(stems))
    }
    fn export(&self) -> solve::Result {
        // let mut serial = Serial::new();
        // for apex in self.apexes.values() {
        //     apex.serial(&mut serial)?;
        // }
        // let path = self.path.string()?;
        // let data = serde_json::to_string(&serial)?;
        // fs::write(path, data)?;
        Ok(Gain::None)
    }
    fn import(&mut self) -> adapt::Result {
        // let deserializer = self.deserializer.as_ref().ok_or("missing deserializer")?;
        // let path = self.path.string()?;
        // let file = File::open(path)?;
        // let reader = BufReader::new(file);
        // let serial: Serial = serde_json::from_reader(reader)?;
        // self.dump = String::new();
        // for (id, part) in &serial.parts {
        //     if let Ok(apex) = deserializer.deserialize(part) {
        //         self.apexes.insert(id.into(), apex);
        //         self.dump += &(part.to_owned() + "\n" + "gnid==" + id + "\n");
        //     } else {
        //         panic!("failed to tray part: {}", part)
        //     }
        // }
        // fs::write("/home/julian/delimit/repo/storage/debug.txt", &self.dump)?;
        adapt_ok()
    }
    fn find(&self, _: &str) -> solve::Result {
        // let re = Regex::new(regex)?; //Regex::new(r"(?P<story>Delimit index page)")?;
        // let caps = re.captures(&self.dump).ok_or("no match")?;
        // let start = caps.get(0).unwrap().start();
        // let caps = Regex::new("gnid==([a-zA-Z0-9]{16})")?
        //     .captures_at(&self.dump, start)
        //     .ok_or("no match")?;
        // let id = caps.get(1).unwrap().as_str();
        // let apex = self.apexes.get(id).ok_or("id not found")?.clone();
        // Ok(Gain::Apex(apex))
        Ok(Gain::None)
    }
}

impl Adapt for Bay {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(_) => adapt_ok(),
            Post::Extend(apexes) => self.extend(apexes),
            Post::Import => self.import(),
            _ => no_adapter(post),
        }
    }
}

impl Solve for Bay {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Stems => self.stems(),
            Task::Export => self.export(),
            Task::Find(regex) => self.find(&regex),
            _ => no_solver(),
        }
    }
}

// impl Make for Repo {
//     fn make(&self, _: &Back) -> Self {
//         Self {
//             apexes: self.apexes.clone(),
//             path: self.path.clone(),
//             pool: self.pool.clone(),
//             deserializer: self.deserializer.clone(),
//         }
//     }
// }

// Task::Cmd(name) => match name.as_str() {
//     SAVE => self.save(),
//     _ => Ok(Gain::None),
// },
// post::Form::Cmd(name) => match name.as_str() {
//     TRAY => self.tray(),
//     _ => Ok(Report::None),
// },

// let mut debug = String::new();
// if let Ok(apex) = apex_result {
//     self.apexes.insert(id.into(), apex);
//     all.push_str(string);
//     all.push_str("\n\n");
// }
// // if let Err(_) = apex_result {
// //     all.push_str(&string);
// //     all.push_str(&"\n\n");
// // }

// debug.push_str(&self.apexes.len().to_string());
//         fs::write("/home/julian/delimit/repo/storage/debug.txt", debug)?;
