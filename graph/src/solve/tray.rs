use super::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Tray {
    None,
    Node(Node),
    Nodes(Vec<Node>),
}

impl From<Node> for Tray {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl From<Vec<Node>> for Tray {
    fn from(value: Vec<Node>) -> Self {
        Self::Nodes(value)
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
