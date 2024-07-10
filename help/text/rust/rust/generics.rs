use plain::TextList;

use crate::rust::*;

pub struct Generics {
    pub name: Item,
    pub fields: Vec<Item>,
}

impl Solve for Generics {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (fields, fields_list) = ", ".list();
        fields_list.writer(|pack| {
            for generic in &self.fields {
                pack.unit.items.reactor(pack.reactor).add(generic);
            }
        });
        let (arrowed, arrowed_list) = "".list();
        arrowed_list.writer(|pack| {
            pack.unit
                .items
                .reactor(pack.reactor)
                .add_str("<")
                .add_role(&fields)
                .add_str(">");
        });
        let (text, list) = "".list();
        list.writer(|pack| {
            pack.unit
                .items
                .reactor(pack.reactor)
                .add(&self.name)
                .add_role(&arrowed);
        });
        text
    }
}

pub fn generics() -> (plain::View<Exact>, Rust<Generics>) {
    let rust = Rust::new(Generics {
        name: plain::string("Untitled"),
        fields: vec![],
    });
    let role = plain::View::Role(Role {
        exact: Exact::Generics(rust.clone()),
        solver: rust.solver(),
    });
    (role, rust)
}
