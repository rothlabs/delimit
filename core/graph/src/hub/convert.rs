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

impl From<&str> for Hub<String> {
    fn from(value: &str) -> Self {
        Hub::Tray(Tray::Base(value.into()))
    }
}

impl<T: Payload> From<T> for Hub<T> {
    fn from(value: T) -> Self {
        Hub::Tray(Tray::Base(value))
    }
}
