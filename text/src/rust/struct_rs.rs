use plain::*;

use crate::rust::*;

pub struct StructRs {
    pub_rs: Leaf<bool>,
    name: Item,
    //fields: Vec<Item>,
}

impl StructRs {}

impl Solve for StructRs {
    type Load = plain::Role;
    fn solve(&self) -> Self::Load {
        let text = "\n".list();
        let pub_rs = "pub".gate(&self.pub_rs); 
        text.list().writer_pack(|pack| {
            pack.unit.items.add_role(&pub_rs, pack.reactor);
        });
        text
    }
}


// plain::TextGate::gate("pub", &self.pub_rs);// 
