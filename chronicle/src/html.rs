// use serde::Serialize;
use crate::text::*;
use graph::*;

use element::Element;

mod attribute;
pub mod element;
mod tag;

// #[derive(Clone, Serialize)]
pub struct Html(UnitSolver<Element, Work>);

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
