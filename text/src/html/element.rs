use super::*;

#[derive(Default, Clone)]
pub struct Element {
    pub tag: Node<String>,
    pub items: Vec<Node<String>>,
    pub close: Option<Node<String>>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| Self {
            tag: self.tag.backed(back),
            items: self.items.backed(back),
            close: self.close.as_ref().map(|close| close.backed(back)),
        })
    }
    pub fn tag(&mut self, tag: impl Into<Node<String>>) -> &mut Self {
        self.tag = tag.into();
        self
    }
    pub fn item(&mut self, item: impl Into<Node<String>>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn close(&mut self, close: impl Into<Node<String>>) -> &mut Self {
        self.close = Some(close.into());
        self
    }
}

impl Grant for Element {
    type Load = Node<String>;
    fn grant(&self) -> Self::Load {
        let mut element = List::new();
        element.separator("\n").item(self.tag.down(1));
        element.extend(self.items.down(1));
        if let Some(close) = &self.close {
            let close = List::new()
                .item("</")
                .item(close.down(1))
                .item(">")
                .value();
            element.item(close);
        }
        element.value()
    }
}

// pub struct Element {
//     pub tag: Stem,
//     pub items: Vec<Stem>,
//     pub close: Option<Stem>,
// }

// impl Element {
//     pub fn hold(tag: &Stem, close: Option<&Stem>) -> Hold<Link<Self>, Role> {
//         let link = Link::make(|back| Self {
//             tag: tag.backed(back),
//             items: vec![],
//             close: close.cloned(),
//         });
//         let role = Role {
//             part: OldPart::Element(link.clone()),
//             form: link.ploy(),
//         };
//         Hold { link, role }
//     }
// }

// impl Grant for Element {
//     type Load = Load;
//     fn grant(&self) -> Load {
//         let Hold { link, role } = "\n".list();
//         link.write(|pack| {
//             let mut element = pack.unit.items.back(pack.back);
//             element.view(&self.tag.grant());
//             // element.use_ploy(&self.tag);
//             for item in &self.items {
//                 element.view(&item.grant());
//                 // element.use_ploy(item);
//             }
//             if let Some(close) = &self.close {
//                 let Hold { link, role } = "".list();
//                 link.write(|pack| {
//                     pack.unit
//                         .items
//                         .back(pack.back)
//                         .str("</")
//                         .view(&close.grant())
//                         //.use_ploy(close)
//                         .str(">");
//                 })
//                 .ok();
//                 element.role(&role);
//             }
//         })
//         .ok();
//         role
//     }
// }

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
