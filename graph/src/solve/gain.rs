use super::*;
use std::result::Result;


/// Value returned by a successful apex solver.
#[derive(Clone, PartialEq, Debug, Hash)]
pub enum Gain {
    None,
    Apex(Apex),
    Apexes(Vec<Apex>),
    Map(Map),
    Imports(Vec<Import>),
    String(String),
    U64(u64),
}

impl Gain {
    fn expected(&self, expected: &str) -> solve::Error {
        Error::WrongGain { expected: expected.into(), found: format!("{:?}", self) }
    }

    /// Move Gain into Ok(...)
    pub fn ok(self) -> solve::Result {
        Ok(self)
    }
    /// Get Apex from Gain.
    pub fn apex(self) -> Result<Apex, crate::Error> {
        match self {
            Self::Apex(apex) => Ok(apex),
            _ => Err(self.expected("Apex"))?
        }
    }
    /// Get `Vec<Apex>` from Gain.
    pub fn apexes(self) -> Result<Vec<Apex>, crate::Error> {
        match self {
            Self::Apexes(apexes) => Ok(apexes),
            _ => Err(self.expected("Apexes"))?,
        }
    }
    /// Get Imports from Gain.
    pub fn imports(self) -> Result<Vec<Import>, crate::Error> {
        match self {
            Self::Imports(imports) => Ok(imports),
            _ => Err(self.expected("Imports"))?,
        }
    }
    /// Get Map from Gain.
    pub fn map(self) -> Result<Map, Error> {
        match self {
            Self::Map(map) => Ok(map),
            _ => Err(self.expected("Map"))?,
        }
    }
    /// Get String from Gain.
    pub fn string(self) -> Result<String, crate::Error> {
        match self {
            Self::String(string) => Ok(string),
            _ => Err(self.expected("String"))?,
        }
    }
    /// Get u64 from Gain.
    pub fn u64(self) -> Result<u64, crate::Error> {
        match self {
            Self::U64(int) => Ok(int),
            _ => Err(self.expected("u64"))?,
        }
    }
}

impl From<String> for Gain {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Apex> for Gain {
    fn from(value: Apex) -> Self {
        Self::Apex(value)
    }
}

impl From<Vec<Apex>> for Gain {
    fn from(value: Vec<Apex>) -> Self {
        Self::Apexes(value)
    }
}

// impl From<&Vec<Apex>> for Gain {
//     fn from(value: &Vec<Apex>) -> Self {
//         Self::Apexes(value.clone())
//     }
// }

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<&Map> for Gain {
    fn from(value: &Map) -> Self {
        Self::Map(value.clone())
    }
}

impl From<&Vec<Import>> for Gain {
    fn from(value: &Vec<Import>) -> Self {
        Self::Imports(value.clone())
    }
}

pub trait IntoGain {
    /// Move into Gain.
    fn gain(self) -> solve::Result;
}

impl<T> IntoGain for T
where
    T: Into<Gain>,
{
    fn gain(self) -> solve::Result {
        Ok(self.into())
    }
}

// fn wrong_gain(variant: &str) -> String {
//     "Wrong Gain variant. Expected: ".to_owned() + variant
// }
