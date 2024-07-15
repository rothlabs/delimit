use super::*;

pub struct Element {
    pub tag: Item,
    pub items: Vec<Item>,
    pub close: Option<Item>,
}

impl Element {
    pub fn new(tag: &Item, close: Option<&Item>) -> Hold<Html<Self>, Item> {
        let link = Html::new(Self {
            tag: tag.clone(),
            items: vec![],
            close: close.cloned(),
        });
        let role = Item::Role(Role {
            actual: Actual::Element(link.clone()),
            method: link.ploy(),
        });
        Hold { link, role }
    }
}

impl Grant for Element {
    type Load = Load;
    fn grant(&self) -> Load {
        let Hold { link, role } = "\n".list();
        link.write(|pack| {
            let mut element = pack.unit.items.back(pack.back);
            element.use_ploy(&self.tag);
            for item in &self.items {
                element.use_ploy(item);
            }
            if let Some(close) = &self.close {
                let Hold { link, role } = "".list();
                link.write(|pack| {
                    pack.unit
                        .items
                        .back(pack.back)
                        .add_str("</")
                        .use_ploy(close)
                        .add_str(">");
                });
                element.add_role(&role);
            }
        });
        role
    }
}
