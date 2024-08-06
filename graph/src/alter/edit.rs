use super::*;

pub struct Edit<T> {
    target: T,
    post: Post,
}

impl<T> Edit<T>
where
    T: DoAlter + Clone,
{
    pub fn insert(&mut self, node: impl Into<Node>) -> &mut Self {
        self.post.insert(node);
        self
    }
    pub fn extend(&mut self, nodes: Vec<impl Into<Node>>) -> &mut Self {
        self.post.extend(nodes);
        self
    }
    pub fn cmd(&self, name: &str) -> alter::Result {
        self.target.alter(Post::cmd(name))
    }
    /// Alter the node with built Post.
    pub fn run(&self) -> alter::Result {
        self.target.alter(self.post.clone())
    }
}

pub trait ToEdit<T> {
    /// Make an editor to setup changes. Call Run to apply changes.
    fn edit(&self) -> Edit<T>;
}

impl<T> ToEdit<T> for T
where
    T: DoAlter + Clone,
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
