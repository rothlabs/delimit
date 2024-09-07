use super::*;

impl From<&Apex> for Apex {
    fn from(value: &Apex) -> Self {
        match value {
            Apex::Void(x) => Apex::Void(x.clone()),
            Apex::String(x) => Apex::String(x.clone()),
            Apex::U8(x) => Apex::U8(x.clone()),
        }
    }
}

impl From<Hub<()>> for Apex {
    fn from(value: Hub<()>) -> Self {
        Apex::Void(value.into())
    }
}

impl From<&str> for Apex {
    fn from(value: &str) -> Self {
        Apex::String(value.into())
    }
}

impl From<Hub<String>> for Apex {
    fn from(value: Hub<String>) -> Self {
        Apex::String(value)
    }
}