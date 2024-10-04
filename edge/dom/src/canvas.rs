use super::*;
use web_sys::js_sys::Reflect;

pub struct Canvas {
    pub object: HtmlCanvasElement,
}

impl Canvas {
    pub fn set_size(&self, width: u32, height: u32) {
        self.object.set_width(width);
        self.object.set_height(height);
    }
    pub async fn gpu<'a>(&self) -> Result<Gpu<'a>> {
        let gpu = Gpu::from_canvas(self.object.clone()).await?;
        Ok(gpu)
    }
    pub fn webgl(&self) -> Result<WebGl> {
        let context_options = Object::new();
        Reflect::set(
            &context_options,
            &"preserveDrawingBuffer".into(),
            &true.into(),
        )?;
        let gl = self
            .object
            .get_context_with_context_options("webgl2", &context_options)?
            .ok_or(no_object())?
            .dyn_into::<WGLRC>()?;
        Ok(WebGl { gl })
    }
}

// use super::*;
// use web_sys::{window, HtmlCanvasElement};

// #[derive(Debug)]
// pub struct Canvas {
//     object: HtmlCanvasElement,
//     gl: WGLRC,
// }

// impl Canvas {
//     pub fn new() -> Node<Self> {
//         let document = window().unwrap().document().unwrap();
//         let object = document
//             .create_element("canvas")
//             .unwrap()
//             .dyn_into::<HtmlCanvasElement>()
//             .unwrap();
//         let gl = object
//             .get_context("webgl2")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<WGLRC>()
//             .unwrap();
//         Self { object, gl }.node().unwrap()
//     }
//     pub fn gpu(&self) -> Gpu {
//         self.gl.clone().into()
//     }
//     pub fn fit_size(&self) {
//         self.object.set_width(self.object.client_width() as u32);
//         self.object.set_height(self.object.client_height() as u32);
//     }
//     pub fn add_to_body(&self) {
//         let window = window().unwrap();
//         let document = window.document().unwrap();
//         let body = document.body().unwrap();
//         body.append_child(&self.object).ok();
//     }
// }

// impl Act for Canvas {
//     async fn act(&self) -> Result<()> {
//         self.fit_size();
//         self.gl.viewport(
//             0,
//             0,
//             self.object.width() as i32,
//             self.object.height() as i32,
//         );
//         Ok(())
//     }
// }
