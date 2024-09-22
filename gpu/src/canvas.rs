use super::*;
use web_sys::{window, HtmlCanvasElement};

#[derive(Debug)]
pub struct Canvas {
    object: HtmlCanvasElement,
    gl: WGLRC,
}

// impl Adapt for Canvas {}

impl Canvas {
    pub fn new() -> Node<Self> {
        let document = window().unwrap().document().unwrap();
        let object = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let gl = object
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WGLRC>()
            .unwrap();
        Self { object, gl }.node().unwrap()
        //Node::make(|_| Ok(Self { object, gl })).unwrap()
        // Node::make(Self { object, gl }).unwrap()
    }
    pub fn gpu(&self) -> Gpu {
        self.gl.clone().into()
    }
    pub fn fit_size(&self) {
        self.object.set_width(self.object.client_width() as u32);
        self.object.set_height(self.object.client_height() as u32);
    }
    pub fn add_to_body(&self) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.append_child(&self.object).ok();
    }
}

impl Act for Canvas {
    // fn back(&mut self, _: &Back) -> Result<()> {
    //     Ok(())
    // }
    async fn act(&self) -> Result<()> {
        self.fit_size();
        self.gl.viewport(
            0,
            0,
            self.object.width() as i32,
            self.object.height() as i32,
        );
        Ok(())
    }
}

impl Reckon for Canvas {}
