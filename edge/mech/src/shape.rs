use derive_node::Digest;
use serde::Serialize;

use super::*;

#[derive(Builder, Debug, Digest, Serialize)]
#[builder(pattern = "owned")]
pub struct Shape {
    test: Hub<u8>,
    // controls: Grc<gpu::Buffer>,
    // plots: Node<gpu::Dispatcher>,
}

impl Solve for Shape {
    type Base = ();
}

impl Adapt for Shape {
    
}