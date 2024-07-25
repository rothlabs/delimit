use serde::Serialize;

use crate::*;

/// Work that holds a load. The most simple work that allows read, write, and copy of the load.
#[derive(Serialize)]
pub struct Ace<L> {
    load: L,
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

impl<L> WriteLoadWork for Ace<L> {
    type Item = L;
    fn write_load_work<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T {
        write(&mut self.load)
    }
}

impl<L> DoReact for Ace<L> {
    fn do_react(&mut self, _: &Meta) -> react::Result {
        Ok(())
    }
}

// impl<L> DoGrant for Ace<L> 
// where 
//     L: Clone,
// {
//     type Load = L;
//     fn do_grant(&mut self, _: &Back) -> Self::Load {
//         self.load.clone()
//     }
// }

// impl<L> Clear for Ace<L> {
//     fn clear(&mut self) {}
// }


// ///////// TODO: replacing FromItem?!?!
// impl<L> From<L> for Ace<L> {
//     fn from(load: L) -> Self {
//         Self { load }
//     }
// }
//             // impl From<String> for Ace<String> {
//             //     fn from(load: String) -> Self {
//             //         Self { load: load.clone() }
//             //     }
//             // }
//             impl<'a> From<&'a str> for Ace<String> {
//                 fn from(load: &'a str) -> Self {
//                     Self { load: load.into() }
//                 }
//             }
