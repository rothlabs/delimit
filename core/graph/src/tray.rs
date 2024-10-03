use super::*;
use serde::Serialize;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tray<T> {
    Path(Path),
    Base(T),
}

impl<T: HashGraph> HashGraph for Tray<T> {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Path(path) => path.hash(state),
            Self::Base(data) => data.hash_graph(state),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected:?}, found: {found:?})")]
    WrongVariant { expected: String, found: String },
}

impl<T> Tray<T>
where
    T: Debug,
{
    pub fn wrong_variant(&self, expected: &str) -> Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }
    }
    pub fn path(&self) -> Option<&Path> {
        match self {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }
}

// impl<T: Hash> Hash for Tray<T> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         match self {
//             Self::Path(path) => path.hash(state),
//             Self::Base(data) => data.hash(state),
//         }
//     }
// }
