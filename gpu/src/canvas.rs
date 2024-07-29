use super::*;
use web_sys::{window, HtmlCanvasElement};

pub struct Canvas {
    object: HtmlCanvasElement,
    gl: WGLRC,
}

impl Canvas {
    pub fn link() -> Agent<Self> {
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
        Agent::new(Self { object, gl })
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
    type Load = ();
    fn act(&self) -> Self::Load {
        self.fit_size();
        self.gl.viewport(
            0,
            0,
            self.object.width() as i32,
            self.object.height() as i32,
        );
    }
}

impl React for Canvas {
    fn react(&self, _: &Meta) -> react::Result {
        self.act();
        Ok(())
    }
}


        // let window = window().unwrap();
        // let memo = format!("width: {}", self.element.width());
        // window.alert_with_message(&memo).ok();

        // self.element
        //     .get_context("webgl2")
        //     .unwrap()
        //     .unwrap()
        //     .dyn_into::<WGLRC>()
        //     .unwrap()
        //     .into()

// impl Default for Canvas {
//     fn default() -> Self {
//         let document = window().unwrap().document().unwrap();
//         let element = document
//             .create_element("canvas")
//             .unwrap()
//             .dyn_into::<HtmlCanvasElement>()
//             .unwrap();
//         Self { element }
//     }
// }

// use super::*;
// use wasm_bindgen::prelude::*;

// pub struct Canvas {
//     element: HtmlCanvasElement,
//     wglrc: WGLRC,
// }

// impl Canvas {
//     pub fn new() -> Self {
//         Self::default()
//     }
//     pub fn gl(&self) -> Base {
//         Base::new(&self.wglrc)
//     }
//     pub fn fit_size(&self) {
//         self.element.set_width(self.element.client_width() as u32);
//         self.element.set_height(self.element.client_height() as u32);
//     }
// }

// impl Default for Canvas {
//     fn default() -> Self {
//         let document = window().unwrap().document().unwrap();
//         let element = document
//             .create_element("canvas")
//             .unwrap()
//             .dyn_into::<HtmlCanvasElement>()
//             .unwrap();
//         let wglrc = element
//             .get_context("webgl2")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<WGLRC>()
//             .unwrap();
//         Self { element, wglrc }
//     }
// }
