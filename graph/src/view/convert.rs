use super::*;

impl<'a> From<&'a mut Hub<String>> for View<'a> {
    fn from(x: &'a mut Hub<String>) -> Self {
        Self::String(x)
    }
}