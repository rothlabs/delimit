use crate::*;

pub struct Gate<T, O> {
    pub active: T,
    pub default: T,
    pub on: O,
}

// impl<L> Gate<L> {
//     fn new(load: L, default: L) -> Self {
//         Self {
//             on: BareLeaf::new(true)
//         }
//     }
// }

impl<T, O> Solve for Gate<T, O> 
where 
    T: Solve,
    O: Solve<Load = bool>, 
{
    type Load = T::Load;
    fn solve(&self) -> T::Load {
        if self.on.solve() {
            self.active.solve()
        } else {
            self.default.solve()
        }
    }
}