use plain::*;

use crate::rust::*;

pub struct StructRs {
    pub_rs: Leaf<bool>,
    name: Item,
    fields: Vec<Item>,
}

impl StructRs {
    fn header(&self, pack: &mut WriterPack<List>) {
        let pub_rs = "pub".gate(&self.pub_rs); 
        pack.unit.items
            .add_role(&pub_rs, pack.reactor)
            .add(self.name.item(pack.reactor))
            .add_str("{");
    }
    fn fields(&self, pack: &mut WriterPack<List>) {
        for field in &self.fields {
            pack.unit.items.add(field.item(pack.reactor));
        }
    }
    fn whole(&self, pack: &mut WriterPack<List>, header: &Load, fields: &Load) {
        pack.unit.items
            .add_role(header, pack.reactor)
            .add_role(fields, pack.reactor)
            .add_str("}");
    }
}

impl Solve for StructRs {
    type Load = Load;
    fn solve(&self) -> Self::Load {
        let (header, header_list) = " ".list();
        header_list.writer_pack(|pack| self.header(pack));
        let (fields, field_list) = ",\n    ".list();
        field_list.writer_pack(|pack| self.fields(pack));
        let (text, list) = "\n".list();
        list.writer_pack(|pack| self.whole(pack, &header, &fields));
        text
    }
}


// plain::TextGate::gate("pub", &self.pub_rs);// 
