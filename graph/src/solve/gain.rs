use super::*;

/// Value returned by a successful hub solver.
#[derive(Clone, PartialEq, Hash, Debug)]
pub enum Gain<T>
where
    T: 'static + Payload,
{
    // for units
    None,
    Hub(Hub<T>),
    String(String),
    U64(u64),
    // for graph internals
    Imports(Vec<Import>),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected:?}, found: {found:?})")]
    WrongVariant { expected: String, found: String },
}

impl<T> Gain<T>
where
    T: Payload,
{
    /// Emit `WrongVariant` error.
    fn wrong_variant(&self, expected: &str) -> solve::Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }
        .into()
    }
    /// Move Gain into Ok(...)
    pub fn ok(self) -> Result<Gain<T>> {
        Ok(self)
    }
    /// Get Hub from Gain.
    pub fn hub(self) -> crate::Result<Hub<T>> {
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

impl<T: Payload> From<String> for Gain<T> {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl<T: Payload> From<Hub<T>> for Gain<T> {
    fn from(value: Hub<T>) -> Self {
        Self::Hub(value)
    }
}

impl<T: Payload> From<u64> for Gain<T> {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl<T: Payload> From<&Vec<Import>> for Gain<T> {
    fn from(value: &Vec<Import>) -> Self {
        Self::Imports(value.clone())
    }
}

pub trait IntoGain<T: Payload> {
    /// Move into Gain.
    fn gain(self) -> Result<Gain<T>>;
}

impl<G, T> IntoGain<T> for G
where
    G: Into<Gain<T>>,
    T: 'static + Payload,
{
    fn gain(self) -> Result<Gain<T>> {
        Ok(self.into())
    }
}
