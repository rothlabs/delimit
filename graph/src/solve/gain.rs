use super::*;
use std::result::Result;

/// Value returned by a successful node solver.
#[derive(Clone, PartialEq, Debug)]
pub enum Gain {
    None,
    Node(Node),
    Nodes(Vec<Node>),
    String(String),
    U64(u64),
}

impl Gain {
    /// Move Gain into Ok(...)
    pub fn ok(self) -> solve::Result {
        Ok(self)
    }
    /// Get Node from Gain.
    pub fn node(self) -> Result<Node, Error> {
        match self {
            Self::Node(node) => Ok(node),
            _ => Err(wrong_gain("Node"))?,
        }
    }
    /// Get `Vec<Node>` from Gain.
    pub fn nodes(self) -> Result<Vec<Node>, Error> {
        match self {
            Self::Nodes(nodes) => Ok(nodes),
            _ => Err(wrong_gain("Nodes"))?,
        }
    }
    /// Get String from Gain.
    pub fn string(self) -> Result<String, Error> {
        match self {
            Self::String(string) => Ok(string),
            _ => Err(wrong_gain("String"))?,
        }
    }
    /// Get u64 from Gain.
    pub fn u64(self) -> Result<u64, Error> {
        match self {
            Self::U64(int) => Ok(int),
            _ => Err(wrong_gain("u64"))?,
        }
    }
}

impl From<Node> for Gain {
    fn from(value: Node) -> Self {
        Self::Node(value)
    }
}

impl From<Vec<Node>> for Gain {
    fn from(value: Vec<Node>) -> Self {
        Self::Nodes(value)
    }
}

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

pub trait IntoGain {
    /// Move into Gain.
    fn gain(self) -> Gain;
}

impl<T> IntoGain for T
where
    T: Into<Gain>,
{
    fn gain(self) -> Gain {
        self.into()
    }
}

fn wrong_gain(variant: &str) -> String {
    "Wrong Gain variant. Expected: ".to_owned() + variant
}

// pub fn wrong_gain(variant: &str) -> solve::Result {
//     Err("Wrong Gain variant. Expected: ".to_owned() + variant)?
// }
