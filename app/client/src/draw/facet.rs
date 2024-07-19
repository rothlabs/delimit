use super::*;

pub struct Facets {
    // arrays: Vec<Array>,
}

impl Facets {
    pub fn new() -> Hold<Link<Self>, Role> {
        let link = Link::new(Self{
            // arrays: vec![]
        });
        let role = Role {
            part: Part::Facets(link.clone()),
            form: link.ploy(),
        };
        Hold { link, role }
    }
}

impl Grant for Facets {
    type Load = ();
    fn grant(&self) -> Self::Load {

    }
}

// type Array = array::Role<f32>;