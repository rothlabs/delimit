use crate::*;

pub trait ToLeaf<T> {
    /// Clone to new Leaf.
    fn leaf(&self) -> Leaf<T>;
}

impl ToLeaf<String> for str {
    /// Clone to Leaf.
    fn leaf(&self) -> Leaf<String> {
        Leaf::new(self.to_owned())
        // Leaf::new(Tray::String(self.into()))
    }
}

pub trait IntoLeaf<T> {
    /// Move into Leaf.
    fn into_leaf(self) -> Leaf<T>;
}

impl<T: 'static + SendSync> IntoLeaf<T> for T {
    fn into_leaf(self) -> Leaf<T> {
        Leaf::new(self)
    }
}

impl<T: 'static + SendSync> From<T> for Leaf<T> {
    fn from(value: T) -> Self {
        Leaf::new(value)
    }
}


// pub trait VecIntoLeaf<T> {
//     /// Move into Leaf.
//     fn leaf(self) -> Leaf<Vec<T>>;
// }

// impl<T: Payload> VecIntoLeaf<T> for Vec<T> {
//     fn leaf(self) -> Leaf<Vec<T>> {
//         Leaf::new(self)
//     }
// }

// impl IntoLeaf<Vec<f32>> for Vec<f32> {
//     fn leaf(self) -> Leaf<Vec<f32>> {
//         Leaf::new(self)
//     }
// }

// impl IntoLeaf<i32> for i32 {
//     fn leaf(self) -> Leaf<i32> {
//         Leaf::new(self)
//     }
// }

// impl<T: Payload> IntoLeaf<T> for Vec<T> {
//     fn leaf(self) -> Leaf<Vec<T>> {
//         Leaf::new(self)
//     }
// }
