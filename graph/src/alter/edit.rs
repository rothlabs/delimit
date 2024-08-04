use super::*;

pub struct Edit<T> {
    target: T,
    post: Post,
}

impl<T> Edit<T>
where
    T: Alter + Clone,
{
    pub fn insert(&mut self, node: impl Into<Node>) -> &mut Self {
        self.post.insert(node.into());
        self
    }
    /// Alter the node with built Post.
    pub fn run(&mut self) -> alter::Result {
        self.target.alter(self.post.clone())
    }
}

pub trait ToEdit<T> {
    /// Make an editor to setup changes. Call Run to apply changes.
    fn edit(&self) -> Edit<T>;
}

impl<T> ToEdit<T> for T
where
    T: Alter + Clone,
{
    fn edit(&self) -> Edit<T> {
        Edit {
            target: self.clone(),
            post: Post::new(),
        }
    }
}

// pub fn insert(&mut self, nodes: Vec<Node>) -> alter::Result {
//     let post = Post::new().insert(nodes).clone();
//     self.target.alter(post)
// }
