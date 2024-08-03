use super::*;

pub struct Query<T> {
    target: T,
}

impl<T> Query<T>
where
    T: Solve + Clone,
{
    pub fn node(&self) -> node::Result {
        match self.target.solve(Task::Node)? {
            Tray::Node(node) => Ok(node),
            _ => Err("not a node".into()),
        }
    }
    // pub fn serial(&self) -> serial::Result {
    //     self.target.solve(Task::Serial)?;
    //     Ok(())
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
        Query {
            target: self.clone(),
        }
    }
}



// pub fn new(item: &T) -> Self {
//     Self { item: item.clone() }
// }

// pub fn load(&self) -> load::Result {
//     match self.item.solve()? {
//         Tray::Load(load) => Ok(load),
//         _ => Err("not a load".into())
//     }
// }
