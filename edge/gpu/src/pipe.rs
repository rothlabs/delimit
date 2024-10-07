pub use compute::*;
pub use layout::*;
pub use render::RenderBuilder;
pub use fragment::*;
pub use vertex::VertexBuilder;

use super::*;

mod compute;
mod layout;
mod render;
mod fragment;
pub mod vertex;

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Primitive {
    topology: PrimitiveTopology,
    #[builder(default)]
    strip_index_format: Option<IndexFormat>,
    #[builder(default)]
    front_face: FrontFace,
    #[builder(default)]
    cull_mode: Option<Face>,
    #[builder(default)]
    unclipped_depth: bool,
    #[builder(default)]
    polygon_mode: PolygonMode,
    #[builder(default)]
    conservative: bool,
}

impl PrimitiveBuilder {
    pub fn make(self) -> Result<PrimitiveState> {
        let built = self.build()?;
        let out = PrimitiveState { 
            topology: built.topology, 
            strip_index_format: built.strip_index_format, 
            front_face: built.front_face, 
            cull_mode: built.cull_mode, 
            unclipped_depth: built.unclipped_depth, 
            polygon_mode: built.polygon_mode, 
            conservative: built.conservative 
        };
        Ok(out)
    }
}