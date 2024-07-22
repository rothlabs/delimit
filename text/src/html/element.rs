use super::*;

pub struct Element {
    pub tag: Stem,
    pub items: Vec<Stem>,
    pub close: Option<Stem>,
}

impl Element {
    pub fn hold(tag: &Stem, close: Option<&Stem>) -> Hold<Link<Self>, Role> {
        let link = Link::make(|back| Self {
            tag: tag.backed(back),
            items: vec![],
            close: close.cloned(),
        });
        let role = Role {
            part: Part::Element(link.clone()),
            form: link.ploy(),
        };
        Hold { link, role }
    }
}

impl Grant for Element {
    type Load = Load;
    fn grant(&self) -> Load {
        let Hold { link, role } = "\n".list();
        link.write(|pack| {
            let mut element = pack.unit.items.back(pack.back);
            element.view(&self.tag.grant());
            // element.use_ploy(&self.tag);
            for item in &self.items {
                element.view(&item.grant());
                // element.use_ploy(item);
            }
            if let Some(close) = &self.close {
                let Hold { link, role } = "".list();
                link.write(|pack| {
                    pack.unit
                        .items
                        .back(pack.back)
                        .str("</")
                        .view(&close.grant())
                        //.use_ploy(close)
                        .str(">");
                });
                element.role(&role);
            }
        });
        role
    }
}

// impl Grant for Element {
//     type Load = Load;
//     fn grant(&self) -> Load {
//         let Hold { link, role } = "\n".list();
//         link.write(|pack| {
//             let mut element = pack.unit.items.back(pack.back);
//             element.push(&self.tag.grant());
//             // element.use_ploy(&self.tag);
//             for item in &self.items {
//                 element.push(&item.grant());
//                 // element.use_ploy(item);
//             }
//             if let Some(close) = &self.close {
//                 let Hold { link, role } = "".list();
//                 link.write(|pack| {
//                     pack.unit
//                         .items
//                         .back(pack.back)
//                         .str("</")
//                         .push(&close.grant())
//                         //.use_ploy(close)
//                         .str(">");
//                 });
//                 element.add_role(&role);
//             }
//         });
//         role
//     }
// }
