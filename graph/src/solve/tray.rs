use super::*;

/// Value returned by a successful node solver.
#[derive(Clone, PartialEq, Debug)]
pub enum Tray {
    None,
    Node(Node),
    Nodes(Vec<Node>),
    String(String),
    U64(u64),
}

impl Tray {
    /// Move Tray into Ok(...)
    pub fn ok(self) -> solve::Result {
        Ok(self)
    }
    /// Try to get Node from Tray.
    pub fn node(&self) -> node::Result {
        match self {
            Self::Node(node) => Ok(node.clone()),
            _ => Err("No node.")?,
        }
    }
    /// Try to get String from Tray.
    pub fn string(&self) -> result::Result<String, Error> {
        match self {
            Self::String(string) => Ok(string.clone()),
            _ => Err("No string.")?,
        }
    }
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
    /// Move into Tray.
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
