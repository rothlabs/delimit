use super::*;

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
    // Usize(usize),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected:?}, found: {found:?})")]
    WrongVariant { expected: String, found: Gain },
}

impl Gain {
    /// Emit `WrongVariant` error.
    fn wrong_variant(&self, expected: &str) -> solve::Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: self.clone(),
        }
        .into()
    }
    /// Move Gain into Ok(...)
    pub fn ok(self) -> Result<Gain> {
        Ok(self)
    }
    /// Get Apex from Gain.
    pub fn apex(self) -> crate::Result<Apex> {
        match self {
            Self::Apex(apex) => Ok(apex),
            _ => Err(self.wrong_variant("Apex"))?,
        }
    }
    /// Get `Vec<Apex>` from Gain.
    pub fn apexes(self) -> crate::Result<Vec<Apex>> {
        match self {
            Self::Apexes(apexes) => Ok(apexes),
            _ => Err(self.wrong_variant("Apexes"))?,
        }
    }
    /// Get Imports from Gain.
    pub fn imports(self) -> crate::Result<Vec<Import>> {
        match self {
            Self::Imports(imports) => Ok(imports),
            _ => Err(self.wrong_variant("Imports"))?,
        }
    }
    /// Get Map from Gain.
    pub fn map(self) -> crate::Result<Map> {
        match self {
            Self::Map(map) => Ok(map),
            _ => Err(self.wrong_variant("Map"))?,
        }
    }
    /// Get String from Gain.
    pub fn string(self) -> crate::Result<String> {
        match self {
            Self::String(string) => Ok(string),
            _ => Err(self.wrong_variant("String"))?,
        }
    }
    /// Get u64 from Gain.
    pub fn u64(self) -> crate::Result<u64> {
        match self {
            Self::U64(int) => Ok(int),
            _ => Err(self.wrong_variant("u64"))?,
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

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

// impl From<usize> for Gain {
//     fn from(value: usize) -> Self {
//         Self::Usize(value)
//     }
// }

impl From<Map> for Gain {
    fn from(value: Map) -> Self {
        Self::Map(value)
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
    fn gain(self) -> Result<Gain>;
}

impl<T> IntoGain for T
where
    T: Into<Gain>,
{
    fn gain(self) -> Result<Gain> {
        Ok(self.into())
    }
}
