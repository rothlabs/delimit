use super::*;
use anyhow::anyhow;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected}, found: {found})")]
    WrongVariant { expected: String, found: String },
    #[error("index out of bounds: (length: {length}, index: {index})")]
    IndexOutOfBounds { length: usize, index: usize },
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub enum Aim {
    Key(Key),
    Index(usize),
}

impl Aim {
    pub fn wrong_variant(&self, expected: &str) -> Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }
    }
    pub fn index_out_of_bounds(&self, length: usize) -> Error {
        if let Self::Index(i) = self {
            Error::IndexOutOfBounds { length, index: *i }
        } else {
            anyhow!("Wrong Aim variant for IndexOutOfBounds!").into()
        }
    }
}

impl From<&Aim> for Aim {
    fn from(aim: &Aim) -> Self {
        aim.clone()
    }
}

impl From<&str> for Aim {
    fn from(value: &str) -> Self {
        Self::Key(value.into())
    }
}

impl From<&String> for Aim {
    fn from(value: &String) -> Self {
        Self::Key(value.clone())
    }
}

impl From<usize> for Aim {
    fn from(value: usize) -> Self {
        Self::Index(value)
    }
}

// impl<'a> From<&'a [Key]> for Aim<'a> {
//     fn from(value: &'a [Key]) -> Self {
//         Self::Keys(value)
//     }
// }

// impl<'a> From<&'a Vec<Key>> for Aim<'a> {
//     fn from(value: &'a Vec<Key>) -> Self {
//         Self::Keys(value)
//     }
// }
