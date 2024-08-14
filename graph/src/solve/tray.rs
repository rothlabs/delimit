use super::*;
use std::result::Result;

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
    /// Get Node from Tray.
    pub fn node(self) -> Result<Node, Error> {
        match self {
            Self::Node(node) => Ok(node),
            _ => Err(wrong_tray("Node"))?,
        }
    }
    /// Get `Vec<Node>` from Tray.
    pub fn nodes(self) -> Result<Vec<Node>, Error> {
        match self {
            Self::Nodes(nodes) => Ok(nodes),
            _ => Err(wrong_tray("Nodes"))?,
        }
    }
    /// Get String from Tray.
    pub fn string(self) -> Result<String, Error> {
        match self {
            Self::String(string) => Ok(string),
            _ => Err(wrong_tray("String"))?,
        }
    }
    /// Get u64 from Tray.
    pub fn u64(self) -> Result<u64, Error> {
        match self {
            Self::U64(int) => Ok(int),
            _ => Err(wrong_tray("u64"))?,
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

impl From<u64> for Tray {
    fn from(value: u64) -> Self {
        Self::U64(value)
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

fn wrong_tray(variant: &str) -> String {
    "Wrong Tray variant. Expected: ".to_owned() + variant
}

// pub fn wrong_tray(variant: &str) -> solve::Result {
//     Err("Wrong Tray variant. Expected: ".to_owned() + variant)?
// }
