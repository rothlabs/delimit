use super::*;

impl<'a> From<&'a mut Apex> for View<'a> {
    fn from(value: &'a mut Apex) -> Self {
        match value {
            Apex::Void(x) => View::Void(x),
            Apex::String(x) => View::String(x),
            Apex::U8(x) => View::U8(x),
            Apex::I32(x) => View::I32(x),
            Apex::F64(x) => View::F64(x),
            Apex::Vu8(x) => View::Vu8(x),
            Apex::Vu16(x) => View::Vu16(x),
            Apex::Vf32(x) => View::Vf32(x),
            Apex::Vf64(x) => View::Vf64(x),
        }
    }
}

impl<'a> From<&'a mut Hub<String>> for View<'a> {
    fn from(x: &'a mut Hub<String>) -> Self {
        Self::String(x)
    }
}

impl<'a> From<&'a mut Hub<i32>> for View<'a> {
    fn from(x: &'a mut Hub<i32>) -> Self {
        Self::I32(x)
    }
}

impl<'a> From<&'a mut Hub<f64>> for View<'a> {
    fn from(x: &'a mut Hub<f64>) -> Self {
        Self::F64(x)
    }
}
