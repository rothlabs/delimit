use super::*;

pub struct Window {
    object: web_sys::Window,
}

impl Window {
    pub fn new() -> Result<Self> {
        Ok(Self {
            object: window().ok_or(anyhow!("no window"))?
        })
    }
    pub fn body(&self) -> Result<Element> {
        let document = self.object.document().ok_or(anyhow!("no document"))?;
        let object = document.body().ok_or(anyhow!("no body"))?;
        Ok(Element { document, object })
    }
}