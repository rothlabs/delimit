use super::*;

pub struct Query<T> {
    item: T,
}

impl<T> Query<T>
where
    T: Solve + Clone,
{
    pub fn new(item: &T) -> Self {
        Self { item: item.clone() }
    }
    pub fn node(&self) -> node::Result {
        match self.item.solve(Task::Main)? {
            Tray::Node(node) => Ok(node),
            _ => Err("not a node".into()),
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
    T: Solve + Clone,
{
    fn query(&self) -> Query<T> {
        Query { item: self.clone() }
    }
}
