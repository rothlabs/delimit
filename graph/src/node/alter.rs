use super::*;

impl Node {
    pub fn insert(&self, node: impl Into<Node>) -> adapt::Result {
        self.adapt(Post::Insert(node.into()))
    }
    pub fn extend(&self, nodes: Vec<impl Into<Node>>) -> adapt::Result {
        self.adapt(Post::Extend(nodes.into_iter().map(|node| node.into()).collect()))
    }
    pub fn import(&self) -> adapt::Result {
        self.adapt(Post::Import)
    }
}