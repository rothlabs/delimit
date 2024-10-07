use super::*;

impl<T: Clone> From<&Hub<T>> for Hub<T>
where
    T: Payload,
{
    fn from(value: &Hub<T>) -> Self {
        value.clone()
    }
}

impl<T, U> From<Node<U>> for Hub<T>
where
    T: 'static + Payload,
    U: 'static + Unit<Base = T>,
{
    fn from(node: Node<U>) -> Self {
        Hub::Ploy(node.as_ploy())
    }
}

impl<T> From<Ploy<T>> for Hub<T>
where
    T: Payload,
{
    fn from(ploy: Ploy<T>) -> Self {
        Hub::Ploy(ploy)
    }
}

impl<T> From<Leaf<T>> for Hub<T>
where
    T: Payload,
{
    fn from(leaf: Leaf<T>) -> Self {
        Hub::Leaf(leaf)
    }
}

impl<T> From<&Leaf<T>> for Hub<T>
where
    T: Payload,
{
    fn from(value: &Leaf<T>) -> Self {
        Hub::Leaf(value.clone())
    }
}

impl From<Vec<u8>> for Hub<Vec<u8>> {
    fn from(value: Vec<u8>) -> Self {
        Hub::Leaf(Leaf::new(value))
    }
}

impl From<Vec<u16>> for Hub<Vec<u16>> {
    fn from(value: Vec<u16>) -> Self {
        Hub::Leaf(Leaf::new(value))
    }
}

impl From<Vec<u32>> for Hub<Vec<u32>> {
    fn from(value: Vec<u32>) -> Self {
        Hub::Leaf(Leaf::new(value))
    }
}


impl From<Vec<f32>> for Hub<Vec<f32>> {
    fn from(value: Vec<f32>) -> Self {
        Hub::Leaf(Leaf::new(value))
    }
}

impl From<Vec<f64>> for Hub<Vec<f64>> {
    fn from(value: Vec<f64>) -> Self {
        Hub::Leaf(Leaf::new(value))
    }
}

impl From<&str> for Hub<String> {
    fn from(value: &str) -> Self {
        Hub::Tray(Tray::Base(value.into()))
        //Hub::Tray(Tray::String(value.to_owned()))
    }
}

impl From<String> for Hub<String> {
    fn from(value: String) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}

impl From<u32> for Hub<u32> {
    fn from(value: u32) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}

impl From<i32> for Hub<i32> {
    fn from(value: i32) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}

impl From<f64> for Hub<f64> {
    fn from(value: f64) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}

// pub trait ToHub {
//     type Base: Payload;
//     /// Move into `Hub`
//     fn hub(self) -> Vec<Hub<Self::Base>>;
// }

// impl<T: 'static + Payload + Into<Hub<T>> > ToHub for Vec<T> {
//     type Base = T;
//     fn hub(self) -> Vec<Hub<Self::Base>> {
//         let mut out = vec![];
//         for item in self {
//             out.push(item.into());
//         }
//         out
//     }
// }
