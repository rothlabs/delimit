use super::*;

// impl From<Tray> for Hub {
//     fn from(value: Tray) -> Self {
//         Hub::Tray(value)
//     }
// }

impl<T> From<Leaf<T>> for Hub<T> 
where 
    T: Payload
{
    fn from(leaf: Leaf<T>) -> Self {
        Hub::Leaf(leaf)
    }
}

impl<T> From<Ploy<T>> for Hub<T> 
where 
    T: Payload
{
    fn from(ploy: Ploy<T>) -> Self {
        Hub::Ploy(ploy)
    }
}

impl<T> From<&Leaf<T>> for Hub<T> 
where 
    T: Payload
{
    fn from(value: &Leaf<T>) -> Self {
        Hub::Leaf(value.clone())
    }
}

impl<T: Clone> From<&Hub<T>> for Hub<T> 
where 
    T: Payload
{
    fn from(value: &Hub<T>) -> Self {
        value.clone()
    }
}

impl From<&str> for Hub<String> {
    fn from(value: &str) -> Self {
        Hub::Tray(Tray::Item(value.into()))
        //Hub::Tray(Tray::String(value.to_owned()))
    }
}

impl From<String> for Hub<String> {
    fn from(value: String) -> Self {
        Hub::Tray(Tray::Item(value))
    }
}

impl From<u32> for Hub<u32> {
    fn from(value: u32) -> Self {
        Hub::Tray(Tray::Item(value))
    }
}

impl From<i32> for Hub<i32> {
    fn from(value: i32) -> Self {
        Hub::Tray(Tray::Item(value))
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

// impl From<Vec<f32>> for Hub<Vec<f32>> {
//     fn from(value: Vec<f32>) -> Self {
//         Hub::Leaf(Leaf::new(value))
//     }
// }
