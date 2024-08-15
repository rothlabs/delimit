use super::*;
use std::result::Result;

/// Value returned by a successful apex solver.
#[derive(Clone, PartialEq, Debug)]
pub enum Gain {
    None,
    Apex(Apex),
    Apexes(Vec<Apex>),
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

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

pub trait IntoGain {
    /// Move into Gain.
    fn gain(self) -> Gain;
}

impl<T> IntoGain for T
where
    T: Into<Gain>,
{
    fn gain(self) -> Gain {
        self.into()
    }
}

fn wrong_gain(variant: &str) -> String {
    "Wrong Gain variant. Expected: ".to_owned() + variant
}

// pub fn wrong_gain(variant: &str) -> solve::Result {
//     Err("Wrong Gain variant. Expected: ".to_owned() + variant)?
// }
