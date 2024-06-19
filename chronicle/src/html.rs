use serde::Serialize;

use graph::{Edge, Solve};
use unit::Element;

use super::text::Text;

mod attribute;
mod tag;
pub mod unit;
pub use unit::doc;

#[derive(Clone, Serialize)]
pub struct Html(pub Edge<Element, (), Text>);

impl Solve<(), Text> for Element {
    fn solve(&self, _: ()) -> Option<Text> {
        Some(self.text())
    }
    fn stems(&self) -> Vec<Box<dyn graph::Stem>> {
        vec![]
    }
}

impl Html {
    pub fn text(&self) -> Text {
        self.0.solve(())
    }
}

pub fn html(element: Element) -> Html {
    Html(Edge::new(element))
}
