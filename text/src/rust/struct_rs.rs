use crate::rust::*;

pub struct StructRs {
    //pub_rs: Viewer<bool>,
    name: Item,
    fields: Vec<Item>,
}

impl StructRs {
    
}

impl Solve for StructRs {
    type Load = plain::Role;
    fn solve(&self) -> Self::Load {
        let text = "\n".text_list();
        text.writer_pack(|pack| {
            //"pub".bool_str(self.pub_rs)
            // if self.pub_rs.solve() {

            // } else {

            // }
        });
        plain::list(&text)
    }
}