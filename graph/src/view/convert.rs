use super::*;

impl<'a> From<&'a mut Apex> for View<'a> {
    fn from(value: &'a mut Apex) -> Self {
        match value {
            Apex::Void(x) => View::Void(x),
            Apex::String(x) => View::String(x),
            Apex::U8(x) => View::U8(x),
            Apex::Vu8(x) => View::Vu8(x),
            Apex::Vu16(x) => View::Vu16(x),
            Apex::Vf32(x) => View::Vf32(x),
        }
    }
}

impl<'a> From<&'a mut Hub<String>> for View<'a> {
    fn from(x: &'a mut Hub<String>) -> Self {
        Self::String(x)
    }
}
