use serde::Serialize;

use crate::*;

/// Wrapper around a single Load as opposed to Work that must Grant for Load
#[derive(Serialize)]
pub struct Ace<L> {
    load: L,
}

///////// TODO: replacing FromItem?!?!
            impl<L> From<L> for Ace<L> {
                fn from(load: L) -> Self {
                    Self { load }
                }
            }

impl<L> FromItem for Ace<L> {
    type Item = L;
    fn new(load: Self::Item) -> Self {
        Self { load }
    }
}

impl<L> ToLoad for Ace<L>
where
    L: Clone,
{
    type Load = L;
    fn load(&self) -> Self::Load {
        self.load.clone()
    }
}

impl<L> DoRead for Ace<L> {
    type Item = L;
    fn do_read(&self) -> &Self::Item {
        &self.load
    }
}

impl<L> DoWrite for Ace<L> {
    type Item = L;
    fn do_write<F: FnOnce(&mut Self::Item)>(&mut self, write: F) {
        write(&mut self.load);
    }
}
