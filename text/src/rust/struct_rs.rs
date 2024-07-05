use plain::TextGate;

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
        let text = "\n".text_list();
        let pub_rs = "pub".text_gate(&self.pub_rs);
        text.writer_pack(|pack| {
            // pack.unit.items.add_role(pub_rs, pack.reactor);
        });
        plain::list(&text)
    }
}
