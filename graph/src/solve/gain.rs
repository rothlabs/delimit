use super::*;
use std::result::Result;

/// Value returned by a successful apex solver.
#[derive(Clone, PartialEq, Debug)]
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
    /// Move Gain into Ok(...)
    pub fn ok(self) -> solve::Result {
        Ok(self)
    }
    /// Get Apex from Gain.
    pub fn apex(self) -> Result<Apex, Error> {
        match self {
            Self::Apex(apex) => Ok(apex),
            _ => Err(wrong_gain("Apex"))?,
        }
    }
    /// Get `Vec<Apex>` from Gain.
    pub fn apexes(self) -> Result<Vec<Apex>, Error> {
        match self {
            Self::Apexes(apexes) => Ok(apexes),
            _ => Err(wrong_gain("Apexes"))?,
        }
    }
    /// Get Imports from Gain.
    pub fn imports(self) -> Result<Vec<Import>, Error> {
        match self {
            Self::Imports(imports) => Ok(imports),
            _ => Err(wrong_gain("Imports"))?,
        }
    }
    /// Get Map from Gain.
    pub fn map(self) -> Result<Map, Error> {
        match self {
            Self::Map(map) => Ok(map),
            _ => Err(wrong_gain("Map"))?,
        }
    }
    /// Get String from Gain.
    pub fn string(self) -> Result<String, Error> {
        match self {
            Self::String(string) => Ok(string),
            _ => Err(wrong_gain("String"))?,
        }
    }
    /// Get u64 from Gain.
    pub fn u64(self) -> Result<u64, Error> {
        match self {
            Self::U64(int) => Ok(int),
            _ => Err(wrong_gain("u64"))?,
        }
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

fn wrong_gain(variant: &str) -> String {
    "Wrong Gain variant. Expected: ".to_owned() + variant
}
