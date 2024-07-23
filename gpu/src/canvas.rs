use super::*;

pub struct Canvas {
    element: HtmlCanvasElement,
}

impl Canvas {
    pub fn link() -> Agent<Self> {
        let document = window().unwrap().document().unwrap();
        let element = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        Agent::new(Self { element })
    }
    pub fn gpu(&self) -> Gpu {
        self.element
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WGLRC>()
            .unwrap()
            .into()
    }
    pub fn fit_size(&self) {
        self.element.set_width(self.element.client_width() as u32);
        self.element.set_height(self.element.client_height() as u32);
    }
}

impl Act for Canvas {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.fit_size();
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
