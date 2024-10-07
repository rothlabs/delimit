use super::*;

// impl<'a> From<&'a mut Vec<Hub<()>>> for ViewVec<'a> {
//     fn from(x: &'a mut Vec<Hub<()>>) -> Self {
//         Self::Void(x)
//     }
// }

impl<'a> From<&'a mut Vec<Hub<String>>> for ViewVec<'a> {
    fn from(x: &'a mut Vec<Hub<String>>) -> Self {
        Self::String(x)
    }
}

impl<'a> From<&'a mut Vec<Hub<f64>>> for ViewVec<'a> {
    fn from(x: &'a mut Vec<Hub<f64>>) -> Self {
        Self::F64(x)
    }
}
