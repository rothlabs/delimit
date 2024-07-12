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
        let view = Item::Role(Role {
            exact: Exact::Element(link.clone()),
            solver: link.solver(),
        });
        Hold { link, view }
    }
}

impl Solve for Element {
    type Load = Load;
    fn solve(&self) -> Load {
        let Hold { link, view } = "\n".list();
        link.writer(|pack| {
            let mut element = pack.unit.items.root(pack.root);
            element.add_item(&self.tag);
            for item in &self.items {
                element.add_item(item);
            }
            if let Some(close) = &self.close {
                let Hold { link, view } = "".list();
                link.writer(|pack| {
                    pack.unit
                        .items
                        .root(pack.root)
                        .add_str("</")
                        .add_item(close)
                        .add_str(">");
                });
                element.add_role(&view);
            }
        });
        view
    }
}
