use crate::plain::*;

pub type Gate = graph::Gate<Item, LeafEye<bool>>;
// Leaf<bool>
pub trait TextGate {
    fn gate(self, on: &LeafEye<bool>) -> Role;
}

impl TextGate for &str {
    fn gate(self, on: &LeafEye<bool>) -> Role {
        let text = Text::from_unit(Gate {
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

// pub trait TextGate {
//     fn gate(self, on: &Leaf<bool>) -> Text<Gate>;
// }

// impl TextGate for &str {
//     fn gate(self, on: &Leaf<bool>) -> Text<Gate> {
//         Text::new(Gate {
//             active: Item::Bare(self.into()),
//             default: Item::default(),
//             on: on.clone(),
//         })
//     }
// }
