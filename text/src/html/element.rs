use crate::html::*;
use crate::plain::List;

pub struct Element {
    tag: &'static TagName,
    items: Vec<Item>,
    attributes: Vec<Item>,
}

impl Element {
    pub fn new() -> Self {
        Element::default()
    }
    fn tag(&self, pack: &mut Pack<List>) {
        let mut tag = pack.unit.items.root(pack.root);
        tag.add_str(&self.tag.open);
        for att in &self.attributes {
            tag.add_view(att);
        }
        tag.add_str(">");
    }
    fn full(&self, pack: &mut Pack<List>, tag: &Load) {
        let mut full = pack.unit.items.root(pack.root);
        full.add_role(tag);
        for item in &self.items {
            full.add_view(item);
        }
        full.add_str(&self.tag.close);
    }
}

impl Default for Element {
    fn default() -> Self {
        Self {
            tag: &DOCTYPE,
            items: vec![],
            attributes: vec![],
        }
    }
}

impl Solve for Element {
    type Load = Load;
    fn solve(&self) -> Load {
        let (open_tag, open_tag_list) = " ".list();
        open_tag_list.writer(|pack| self.tag(pack));
        let (text, text_list) = "\n".list();
        text_list.writer(|pack| self.full(pack, &open_tag));
        text
    }
}
