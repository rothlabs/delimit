use serde::Serialize;
use dyn_clone::DynClone;
use erased_serde::serialize_trait_object;

use graph::{Edge, Snap, Solve};
use unit::Element;

use super::text::Text;

mod attribute;
mod tag;
pub mod unit;

#[derive(Clone, Serialize)]
pub struct Html(pub Edge<Element, (), Text>);

impl Solve<(), Text> for Element {
    fn solve(&self, _: ()) -> Option<Text> {
        Some(self.text())
    }
}

impl Html {
    pub fn text(&self) -> Text {
        self.0.solve(())
    }
    // pub fn get(&self) -> Ref<'_, dyn Unit> {
    //     self.0.as_ref().borrow()
    // }
    // // pub fn serialize(&self) -> String {
    // //     self.get().serialize()
    // // }
    // pub fn any(&self) -> &dyn Any {
    //     self
    // }
}

// dyn_clone::clone_trait_object!(Unit);
// serialize_trait_object!(Unit);
// pub trait Unit: DynClone + erased_serde::Serialize {
//     fn text(&self, snap: &Snap) -> Text;
// }

pub fn html(element: Element) -> Html {
    Html(Edge::new(element))
}

pub struct Task {
    pub snap: Snap,
}
