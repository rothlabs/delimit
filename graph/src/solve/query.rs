use super::*;

pub struct Query<T> {
    target: T,
}

impl<T> Query<T>
where
    T: Solve + Clone,
{
    pub fn stems(&self) -> result::Result<Vec<Node>, Error> {
        match self.target.solve(Task::Stems)? {
            Tray::Nodes(nodes) => Ok(nodes),
            _ => Err("not Tray::Nodes".into()),
        }
    }
    pub fn deep_stems(&self) -> result::Result<Vec<Node>, Error> {
        match self.target.solve(Task::Stems)? {
            Tray::Nodes(nodes) => {
                let mut deep: Vec<Node> = nodes
                    .iter()
                    .flat_map(|x| {
                        if let Ok(nodes) = x.query().deep_stems() {
                            return nodes;
                        }
                        vec![]
                    })
                    .collect();
                deep.extend(nodes);
                Ok(deep)
            }
            _ => Err("not Tray::Nodes".into()),
        }
    }
    pub fn find(&self, regex: &str) -> solve::Result {
        self.target.solve(Task::Find(regex.into()))
    }
    pub fn export(&self) -> solve::Result {
        self.target.solve(Task::Export)
    }
}

pub trait ToQuery<T> {
    fn query(&self) -> Query<T>;
}

impl<T> ToQuery<T> for T
where
    T: Solve + Clone,
{
    fn query(&self) -> Query<T> {
        Query {
            target: self.clone(),
        }
    }
}

// pub fn main(&self) -> node::Result {
//     match self.target.solve(Task::Main)? {
//         Tray::Node(node) => Ok(node),
//         _ => Err("not Tray::Node".into()),
//     }
// }

// pub fn serial(&self) -> serial::Result {
//     self.target.solve(Task::Serial)?;
//     Ok(())
// }

// pub fn new(item: &T) -> Self {
//     Self { item: item.clone() }
// }

// pub fn load(&self) -> load::Result {
//     match self.item.solve()? {
//         Tray::Load(load) => Ok(load),
//         _ => Err("not a load".into())
//     }
// }
