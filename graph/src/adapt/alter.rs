use super::*;

pub struct Alter<T> {
    target: T,
    // post: Post,
}

impl<T> Alter<T>
where
    T: AdaptInner + Clone,
{
    pub fn insert(&self, node: impl Into<Node>) -> adapt::Result {
        self.target.adapt(Post::Insert(node.into()))
    }
    pub fn extend(&self, nodes: Vec<impl Into<Node>>) -> adapt::Result {
        self.target.adapt(Post::Extend(
            nodes.into_iter().map(|node| node.into()).collect(),
        ))
    }
    pub fn import(&self) -> adapt::Result {
        self.target.adapt(Post::Import)
    }
}

pub trait ToAlter<T> {
    /// Make an editor to setup changes. Call Run to apply changes.
    fn alter(&self) -> Alter<T>;
}

impl<T> ToAlter<T> for T
where
    T: AdaptInner + Clone,
{
    fn alter(&self) -> Alter<T> {
        Alter {
            target: self.clone(),
            // post: Post::None,
        }
    }
}

// pub fn insert(&mut self, node: impl Into<Node>) -> &mut Self {
//     self.post.insert(node);
//     self
// }
// pub fn extend(&mut self, nodes: Vec<impl Into<Node>>) -> &mut Self {
//     self.post.extend(nodes);
//     self
// }
// pub fn import(&self) -> adapt::Result {
//     self.target.adapt(Post { field: "".into(), form: post::Form::Import })
// }
// /// Alter the node with built Post.
// pub fn run(&self) -> adapt::Result {
//     self.target.adapt(self.post.clone())
// }

// pub fn insert(&mut self, nodes: Vec<Node>) -> alter::Result {
//     let post = Post::new().insert(nodes).clone();
//     self.target.alter(post)
// }
