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
pub struct Html(pub Edge<Element, Task, Text>);

impl Solve<Task, Text> for Element {
    fn solve(&self, task: Task) -> Option<Text> {
        Some(self.text(&task.snap))
    }
}

impl Html {
    pub fn text(&self) -> Text {
        let task = Task{snap: self.0.snap()};
        self.0.solve(task)
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

pub fn html(snap: &Snap, element: Element) -> Html {
    Html(snap.edge(element))
}

pub struct Task {
    pub snap: Snap,
}
