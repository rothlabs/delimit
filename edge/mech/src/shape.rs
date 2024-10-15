// use derive_builder::Builder;

use super::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Shape {
    controls: Grc<gpu::Buffer>,
    plots: Node<gpu::Dispatcher>,
}

impl Solve for Shape {
    type Base = ();
}