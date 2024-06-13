use std::collections::HashSet;

pub struct Flat {
    pub units: HashSet<String>,
}

pub trait Flatten {
    fn flatten(&self, flat: &mut Flat);
}