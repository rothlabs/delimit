use super::*;

pub struct Facets {
    // arrays: Vec<Array>,
}

impl Facets {
    // pub fn new() -> Hold<Link<Self>, Role> {
    //     let link = Link::new(Self{
    //         // arrays: vec![]
    //     });
    //     let role = Role {
    //         part: Part::Facets(link.clone()),
    //         form: link.ploy(),
    //     };
    //     Hold { link, role }
    // }
}

impl Solve for Facets {
    fn solve(&self, _: Task) -> solve::Result {
        Ok(Gain::None)
    }
}

// type Array = array::Role<f32>;
