use crate::*;

pub struct Gate<T, O> {
    pub active: T,
    pub default: T,
    pub on: O,
}

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


// impl<L, O> Gate<L, O>
// where
//     L: Clone,
//     O: Clone,
// {
//     pub fn new(active: &L, default: &L, on: &O) -> Self {
//         Self {
//             active: active.clone(),
//             default: default.clone(),
//             on: on.clone(),
//         }
//     }
// }