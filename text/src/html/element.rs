use super::*;

#[derive(Default)]
pub struct Element {
    pub tag: Value<String>,
    pub items: Vec<Value<String>>,
    pub close: Option<Value<String>>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn link(&self) -> Deuce<Self> {
        Deuce::make(|back| {
            let close = if let Some(close) = &self.close  {
                Some(close.backed(back))
            } else {
                None
            };
            Self { 
                tag: self.tag.backed(back),
                items: self.items.backed(back),
                close, 
            }
        })
    }
    pub fn tag(&mut self, tag: impl Into<Value<String>>) -> &mut Self {
        self.tag = tag.into();
        self 
    }
    pub fn item(&mut self, item: impl Into<Value<String>>) -> &mut Self {
        self.items.push(item.into());
        self
    }
}

impl Grant for Element {
    type Load = Vec<plain::Part>;
    fn grant(&self) -> Self::Load {
        let mut parts = vec![];
        let mut element = List::new();
        element.separator("\n").item(&self.tag);
        element.extend(self.items.clone());
        if let Some(close) = &self.close {
            let close = List::new().item("</").item(close).item(">").link();
            element.item(close.ploy());
            parts.push(plain::Part::List(close));
        }
        let element = element.link();
        // let wow = plain_element.ploy();
        parts.push(plain::Part::List(element));
        parts
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
