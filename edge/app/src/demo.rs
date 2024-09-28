use super::*;

mod nurbs;

pub struct Demo {
    pub gpu: Gpu,
}
impl Demo {
    pub fn new() -> Self {
        let canvas = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(1200);
        canvas.set_height(900);
        Self {
            gpu: Gpu {
                gl: canvas
                    .get_context("webgl2") // canvas.get_context_with_context_options("webgl2", {preserveDrawingBuffer: true})
                    .unwrap()
                    .unwrap()
                    .dyn_into::<WGLRC>()
                    .unwrap(),
            },
        }
    }
}