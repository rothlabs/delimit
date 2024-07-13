use crate::plain::*;

pub type Gate = graph::Gate<Item, SoleView<bool>>;

pub trait TextGate {
    fn gate(self, on: &SoleView<bool>) -> Role;
}

impl TextGate for &str {
    fn gate(self, on: &SoleView<bool>) -> Role {
        let text = Text::new(Gate {
            active: Item::Bare(self.into()),
            default: Item::default(),
            on: on.clone(),
        });
        Role {
            method: text.ploy(),
            actual: Actual::Gate(text),
        }
    }
}
