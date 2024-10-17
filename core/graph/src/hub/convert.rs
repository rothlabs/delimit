use super::*;

impl<T: Clone> From<&Hub<T>> for Hub<T>
// where
//     T: Payload,
{
    fn from(value: &Hub<T>) -> Self {
        value.clone()
    }
}

impl<T> From<Ploy<T>> for Hub<T>
// where
//     T: SendSync,//Payload,
{
    fn from(ploy: Ploy<T>) -> Self {
        Hub::Ploy(ploy)
    }
}

impl<T> From<Gate<T>> for Hub<T>
// where
//     T: Payload,
{
    fn from(gate: Gate<T>) -> Self {
        Hub::Gate(gate)
    }
}

impl<T> From<Leaf<T>> for Hub<T>
// where
//     T: Payload,
{
    fn from(leaf: Leaf<T>) -> Self {
        Hub::Leaf(leaf)
    }
}

impl<T> From<&Leaf<T>> for Hub<T>
// where
//     T: Payload,
{
    fn from(value: &Leaf<T>) -> Self {
        Hub::Leaf(value.clone())
    }
}

impl From<&str> for Hub<String> {
    fn from(value: &str) -> Self {
        Hub::Tray(Tray::Base(value.into()))
    }
}

impl<T> From<T> for Hub<T> {
    fn from(value: T) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}

pub trait ToPloyHub {
    type Base;//: Payload;
    fn hub(&self) -> Hub<Self::Base>;
}

pub trait ToGateHub {
    type Base; //: Payload;
    fn hub(&self) -> Hub<Self::Base>;
}
