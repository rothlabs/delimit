use super::*;

/// Value returned by a successful hub solver.
#[derive(Clone, PartialEq, Hash, Debug)]
pub enum Gain {
    // for units
    None,
    String(String),
    U64(u64),
    // Vf32(Vf32),
    // for graph internals
    Imports(Vec<Import>),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong variant (expected: {expected:?}, found: {found:?})")]
    WrongVariant { expected: String, found: String },
}

impl Gain {
    /// Emit `WrongVariant` error.
    fn wrong_variant(&self, expected: &str) -> solve::Error {
        Error::WrongVariant {
            expected: expected.into(),
            found: format!("{:?}", self),
        }
        .into()
    }
    /// Move Gain into Ok(...)
    pub fn ok(self) -> Result<Gain> {
        Ok(self)
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
    // /// Get `Vec<f32>` from Gain.
    // pub fn vec_f32(self) -> crate::Result<Vec<f32>> {
    //     match self {
    //         Self::Vf32(x) => Ok(x.0),
    //         _ => Err(self.wrong_variant("Vf32"))?,
    //     }
    // }
}

impl From<String> for Gain {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u64> for Gain {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

// impl<T: Payload> From<u64> for Gain<T> {
//     fn from(value: u64) -> Self {
//         Self::U64(value)
//     }
// }

impl From<&Vec<Import>> for Gain {
    fn from(value: &Vec<Import>) -> Self {
        Self::Imports(value.clone())
    }
}

pub trait IntoGain {
    /// Move into Gain.
    fn gain(self) -> Result<Gain>;
}

impl<G> IntoGain for G
where
    G: Into<Gain>,
{
    fn gain(self) -> Result<Gain> {
        Ok(self.into())
    }
}
