pub use compute::*;
pub use fragment::*;
pub use layout::*;
pub use render::RenderBuilder;
pub use vertex::VertexBuilder;

use super::*;

mod compute;
mod fragment;
mod layout;
mod render;
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
            conservative: built.conservative,
        };
        Ok(out)
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Multisample {
    count: u32,
    #[builder(default = "!0")]
    mask: u64,
    #[builder(default)]
    alpha_to_coverage_enabled: bool,
}

impl MultisampleBuilder {
    pub fn make(self) -> Result<MultisampleState> {
        let built = self.build()?;
        let out = MultisampleState {
            count: built.count,
            mask: built.mask,
            alpha_to_coverage_enabled: built.alpha_to_coverage_enabled,
        };
        Ok(out)
    }
}
