use crate::html::*;

pub struct Element {
    pub tag: Item,
    pub items: Vec<Item>,
    pub close: Item,
}

impl Element {
    pub fn new() -> Hold<Html<Self>, Item> {
        let link = Html::new(Self {
            tag: plain::string("html"),
            items: vec![],
            close: plain::string("html"),
        });
        let view = Item::Role(Role {
            exact: Exact::Element(link.clone()),
            solver: link.solver(),
        });
        Hold { link, view }
    }
    // pub fn new() -> Self {
    //     Element::default()
    // }
}

// impl Default for Element {
//     fn default() -> Self {
//         Self {
//             tag: plain::string("html"),
//             items: vec![],
//             close: plain::string("html"),
//         }
//     }
// }

impl Solve for Element {
    type Load = Load;
    fn solve(&self) -> Load {
        let Hold{link, view} = "\n".list();
        link.writer(|pack| {
            let mut element = pack.unit.items.root(pack.root);
            element.add_view(&self.tag);
            for item in &self.items {
                element.add_view(item);
            }
            element.add_str("</").add_view(&self.close).add_str(">");
        });
        view
    }
}

// fn tag(&self, pack: &mut Pack<List>) {
//     let mut tag = pack.unit.items.root(pack.root);
//     tag.add_str(&self.tag.open);
//     for att in &self.attributes {
//         tag.add_view(att);
//     }
//     tag.add_str(">");
// }
// fn full(&self, pack: &mut Pack<List>, tag: &Load) {
//     let mut full = pack.unit.items.root(pack.root);
//     full.add_role(tag);
//     for item in &self.items {
//         full.add_view(item);
//     }
//     full.add_str(&self.tag.close);
// }
