use crate::plain::*;

pub type Gate = graph::Gate<Item, LeafEye<bool>>;

pub trait TextGate {
    fn gate(self, on: &LeafEye<bool>) -> Role;
}

impl TextGate for &str {
    fn gate(self, on: &LeafEye<bool>) -> Role {
        let text = Text::new(Gate {
            active: Item::Bare(self.into()),
            default: Item::default(),
            on: on.clone(),
        });
        Role {
            solver: text.solver(),
            exact: Exact::Gate(text),
        }
    }
}
