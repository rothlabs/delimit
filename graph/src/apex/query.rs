use super::*;

pub enum Query<'a> {
    Key(Key),
    Keys(&'a [Key]),
    Index(usize),
}

impl From<&str> for Query<'_> {
    fn from(value: &str) -> Self {
        Self::Key(value.into())
    }
}

impl From<&String> for Query<'_> {
    fn from(value: &String) -> Self {
        Self::Key(value.clone())
    }
}

impl<'a> From<&'a [Key]> for Query<'a> {
    fn from(value: &'a [Key]) -> Self {
        Self::Keys(value)
    }
}

impl<'a> From<&'a Vec<Key>> for Query<'a> {
    fn from(value: &'a Vec<Key>) -> Self {
        Self::Keys(value)
    }
}

impl From<usize> for Query<'_> {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}
