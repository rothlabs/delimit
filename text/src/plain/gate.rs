use crate::plain::*;

pub type Gate = graph::Gate<Item, BareSole<bool>>;

pub trait TextGate {
    fn gate(self, on: &BareSole<bool>) -> Role;
}

impl TextGate for &str {
    fn gate(self, on: &BareSole<bool>) -> Role {
        let text = Text::new(Gate {
            active: Item::Base(BareSole::Bare(self.into())),//Item::Bare(self.into()),
            default: Item::default(),
            on: on.clone(),
        });
        Role {
            method: text.ploy(),
            actual: Actual::Gate(text),
        }
    }
}
