use super::*;

pub enum Query {
    Key(Key),
    Index(usize),
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        Self::Key(value.into())
    }
}

impl From<usize> for Query {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}
