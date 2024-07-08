use plain::{List, TextGate, TextList};

use crate::rust::*;

pub struct StructRs {
    pub pub_rs: LeafEye<bool>,
    pub name: Item,
    pub fields: Vec<Item>,
}

impl StructRs {
    fn header(&self, pack: &mut Pack<List>) {
        let pub_rs = "pub".gate(&self.pub_rs);
        pack.unit
            .items
            .reactor(pack.reactor)
            .add_role(&pub_rs)
            .add(&self.name)
            .add_str("{");
    }
    fn fields(&self, pack: &mut Pack<List>) {
        for field in &self.fields {
            pack.unit.items.reactor(pack.reactor).add(field);
        }
    }
    fn whole(&self, pack: &mut Pack<List>, header: &Load, fields: &Load) {
        pack.unit
            .items
            .reactor(pack.reactor)
            .add_role(header)
            .add_role(fields)
            .add_str("}");
    }
}

impl Solve for StructRs {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (header, header_list) = " ".list();
        header_list.writer(|pack| self.header(pack));
        let (fields, field_list) = ",\n    ".list();
        field_list.writer(|pack| self.fields(pack));
        let (text, list) = "\n".list();
        list.writer(|pack| self.whole(pack, &header, &fields));
        text
    }
}

pub fn struct_rs() -> (plain::View<Exact>, Rust<StructRs>) {
    let rust = Rust::new(StructRs {
        pub_rs: LeafEye::new(true),
        name: plain::string("Untitled"),
        fields: vec![],
    });
    let role = plain::View::Role(Role {
        exact: Exact::StructRs(rust.clone()),
        solver: rust.solver(),
    });
    (role, rust)
}

// plain::TextGate::gate("pub", &self.pub_rs);//
