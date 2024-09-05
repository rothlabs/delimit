use super::*;

/// Value returned by a successful hub solver.
#[derive(Clone, PartialEq, Debug, Hash)]
pub enum Gain {
    // for units
    None,
    Hub(Hub),
    String(String),
    U64(u64),
    // for graph internals
    Imports(Vec<Import>),
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
    /// Get Hub from Gain.
    pub fn hub(self) -> crate::Result<Hub> {
        match self {
            Self::Hub(hub) => Ok(hub),
            _ => Err(self.wrong_variant("Hub"))?,
        }
    }
    /// Get Imports from Gain.
    pub fn imports(self) -> crate::Result<Vec<Import>> {
        match self {
            Self::Imports(imports) => Ok(imports),
            _ => Err(self.wrong_variant("Imports"))?,
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

impl From<Hub> for Gain {
    fn from(value: Hub) -> Self {
        Self::Hub(value)
    }
}

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
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
