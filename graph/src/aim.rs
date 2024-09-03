use anyhow::anyhow;
use super::*;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected}, found: {found})")]
    WrongVariant{ expected: String, found: String },
    #[error("index out of bounds: (length: {length}, index: {index})")]
    IndexOutOfBounds{length: usize, index: usize },
}

#[derive(Clone, Debug)]
pub enum Aim<'a> {
    Key(Key),
    Keys(&'a [Key]),
    Index(usize),
}

impl Aim<'_> {
    pub fn wrong_variant(&self, expected: &str) -> adapt::Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }.into()
    }
    pub fn index_out_of_bounds(&self, length: usize) -> adapt::Error {
        if let Self::Index(i) = self {
            Error::IndexOutOfBounds{length, index: *i}.into()
        } else {
            anyhow!("Wrong Aim for IndexOutOfBounds!").into()
        }
    }
}

impl From<&str> for Aim<'_> {
    fn from(value: &str) -> Self {
        Self::Key(value.into())
    }
}

impl From<&String> for Aim<'_> {
    fn from(value: &String) -> Self {
        Self::Key(value.clone())
    }
}

impl<'a> From<&'a [Key]> for Aim<'a> {
    fn from(value: &'a [Key]) -> Self {
        Self::Keys(value)
    }
}

impl<'a> From<&'a Vec<Key>> for Aim<'a> {
    fn from(value: &'a Vec<Key>) -> Self {
        Self::Keys(value)
    }
}

impl From<usize> for Aim<'_> {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}
