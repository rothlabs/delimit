// use serde::Serialize;

use crate::*;

/// Work that holds a load. The most simple work that allows read, write, and copy of the load.
// #[derive(Serialize)]
pub struct Ace {
    load: Load,
}

impl FromItem for Ace {
    type Item = Load;
    fn new(load: Self::Item) -> Self {
        Self { load }
    }
}

impl ToLoad for Ace{
    type Load = Load;
    fn load(&self) -> Self::Load {
        self.load.clone()
    }
}

impl DoRead for Ace {
    type Item = Load;
    fn do_read(&self) -> &Self::Item {
        &self.load
    }
}

impl DoReadLoad for Ace {
    fn do_read_load(&self) -> load::ResultRef {
        Ok(&self.load)
    }
}

impl WriteLoadWork for Ace {
    type Item = Load;
    fn write_load_work<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T {
        write(&mut self.load)
    }
}

impl DoReact for Ace {
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
