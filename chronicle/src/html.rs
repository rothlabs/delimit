// use serde::Serialize;
use graph::*;
use crate::text::*;

use element::Element;

mod attribute;
mod tag;
pub mod element;

// #[derive(Clone, Serialize)]
pub struct Html(Solver<Element, Work>);

impl Html {
    // pub fn text(&self) -> Text<List> {
    //     // self.0.solve()
    // }
}

type Work = graph::Work<Task, Load>;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
enum Task {
    #[default]
    String,
}

#[derive(Clone)]
enum Load {
    String(String),
}
