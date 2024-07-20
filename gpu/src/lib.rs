pub use base::Base;
pub use canvas::Canvas;
pub use shader::Shader;

use web_sys::*;

pub mod shader;

mod base;
mod canvas;

pub type WGLRC = WebGl2RenderingContext;
