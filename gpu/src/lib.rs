pub use canvas::Canvas;
pub use base::Base;
pub use shader::Shader;

use web_sys::*;

pub mod shader;

mod canvas;
mod base;

pub type WGLRC = WebGl2RenderingContext;
