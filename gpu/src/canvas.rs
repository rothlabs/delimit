use super::*;

pub struct Canvas {
    element: HtmlCanvasElement,
    gl: WGLRC,
}

impl Canvas {
    pub fn link() -> Agent<Self> {
        let document = window().unwrap().document().unwrap();
        let element = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let gl = element
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WGLRC>()
            .unwrap();
        Agent::new(Self { element, gl })
    }
    pub fn gpu(&self) -> Gpu {
        self.gl.clone().into()
        // self.element
        //     .get_context("webgl2")
        //     .unwrap()
        //     .unwrap()
        //     .dyn_into::<WGLRC>()
        //     .unwrap()
        //     .into()
    }
    pub fn fit_size(&self) {
        self.element.set_width(self.element.client_width() as u32);
        self.element.set_height(self.element.client_height() as u32);
    }
    pub fn add_to_body(&self) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.append_child(&self.element).ok();
    }
}

impl Act for Canvas {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.fit_size();
        // let window = window().unwrap();
        // let memo = format!("width: {}", self.element.width());
        // window.alert_with_message(&memo).ok();
        self.gl.viewport(0, 0, self.element.width() as i32, self.element.height() as i32);
    }
}

impl React for Canvas {
    fn react(&self, _: &Meta) -> ReactResult {
        self.act();
        Ok(())
    }
}

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
