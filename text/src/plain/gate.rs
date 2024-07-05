use crate::plain::*;

pub type Gate = graph::Gate<Item, Leaf<bool>>;

pub trait TextGate {
    fn text_gate(self, on: &Leaf<bool>) -> Text<Gate>;
}

impl TextGate for &str {
    fn text_gate(self, on: &Leaf<bool>) -> Text<Gate> {
        Text::new(Gate {
            active: Item::Bare(self.into()),
            default: Item::default(),
            on: on.clone(),
        })
    }
}

// Text::new(Gate::new(
//     &Item::Bare(self.into()),
//     &Item::default(),
//     on,
// ))
