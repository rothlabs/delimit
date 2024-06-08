use std::rc::Rc;

use serde::{Serialize, Serializer};

#[derive(Clone)]
pub struct StringUnit(pub Rc<String>);
impl Serialize for StringUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}
pub fn string_unit(value: &str) -> StringUnit {
    StringUnit(Rc::new(value.to_owned()))
}