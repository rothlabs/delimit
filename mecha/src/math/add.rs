use super::*;

pub struct Sum {
    pub items: Vec<Item>,
}

impl Sum {
    pub fn new(items: &Vec<Item>) -> Hold<Math<Self>, Item> {
        let link = Math::new(Self {
            items: items.clone(),
        });
        let view = Item::Role(Role {
            exact: Exact::Sum(link.clone()),
            solver: link.solver(),
        });
        Hold { link, view }
    }
}

impl Solve for Sum {
    type Load = Load;
    fn solve(&self) -> Load {
        let mut value = 0.;
        for item in &self.items {
            item.reader(|v| value += v);
        }
        value.into_sole()
    }
}