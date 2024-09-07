use super::*;

impl From<&Apex> for Apex {
    fn from(value: &Apex) -> Self {
        match value {
            Apex::String(x) => Apex::String(x.clone()),
            Apex::U8(x) => Apex::U8(x.clone()),
        }
    }
}

impl<'a> From<&'a mut Apex> for View<'a> {
    fn from(value: &'a mut Apex) -> Self {
        match value {
            Apex::String(x) => View::String(x),
            Apex::U8(x) => View::U8(x),
        }
    }
}