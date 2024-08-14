use super::*;

impl From<Load> for Node {
    fn from(value: Load) -> Self {
        Node::Load(value)
    }
}

impl From<Leaf> for Node {
    fn from(leaf: Leaf) -> Self {
        Node::Leaf(leaf)
    }
}

impl From<Ploy> for Node {
    fn from(ploy: Ploy) -> Self {
        // TODO: find way to not query the node to get rank!
        let rank = match ploy.main() {
            Ok(node) => match node.rank() {
                Some(rank) => rank + 1,
                None => 1,
            },
            _ => 0,
        };
        Node::Ploy(ploy.ranked(rank))
    }
}

impl From<&Leaf> for Node {
    fn from(value: &Leaf) -> Self {
        Node::Leaf(value.clone())
    }
}

impl From<&Node> for Node {
    fn from(value: &Node) -> Self {
        value.clone()
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Node::Load(Load::String(value.to_owned()))
    }
}

impl From<u32> for Node {
    fn from(value: u32) -> Self {
        Node::Load(Load::U32(value))
    }
}

impl From<i32> for Node {
    fn from(value: i32) -> Self {
        Node::Load(Load::I32(value))
    }
}

impl From<Vec<u8>> for Node {
    fn from(value: Vec<u8>) -> Self {
        Node::Leaf(Leaf::new(Load::Vu8(value)))
    }
}

impl From<Vec<u16>> for Node {
    fn from(value: Vec<u16>) -> Self {
        Node::Leaf(Leaf::new(Load::Vu16(value)))
    }
}

impl From<Vec<f32>> for Node {
    fn from(value: Vec<f32>) -> Self {
        Node::Leaf(Leaf::new(Load::Vf32(value)))
    }
}