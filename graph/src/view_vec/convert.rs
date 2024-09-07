use super::*;

impl<'a> From<&'a mut Vec<Hub<String>>> for ViewVec<'a> {
    fn from(x: &'a mut Vec<Hub<String>>) -> Self {
        Self::String(x)
    }
}