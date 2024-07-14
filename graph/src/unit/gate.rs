use crate::*;

pub struct Gate<T, O> {
    pub active: T,
    pub default: T,
    pub on: O,
}

impl<T, O> Grant for Gate<T, O>
where
    T: Grant,
    O: Grant<Load = Ace<bool>>,
{
    type Load = T::Load;
    fn grant(&self) -> T::Load {
        if self.on.grant().load() {
            self.active.grant()
        } else {
            self.default.grant()
        }
    }
}
