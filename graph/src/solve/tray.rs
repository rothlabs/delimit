use super::*;

#[derive(Clone, PartialEq)]
pub enum Tray {
    None,
    Node(Node),
    // Serial(serial::Result),
}

impl From<Node> for Tray {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

pub trait IntoTray {
    fn tray(self) -> Tray;
}

impl<T> IntoTray for T
where
    T: Into<Tray>,
{
    fn tray(self) -> Tray {
        self.into()
    }
}
