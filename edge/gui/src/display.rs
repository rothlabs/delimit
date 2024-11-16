use super::*;
use winit::dpi::PhysicalSize;

#[derive(Clone, Debug)]
pub struct Display {
    pub viewport: Viewport,
    pub window: Grc<Window>,
}

impl Display {
    pub fn new(window: Grc<Window>, viewport: Viewport) -> Self {
        Self { viewport, window }
    }
    pub fn render(&self) -> Result<()> {
        Ok(self.viewport.render()?)
    }
    pub fn resize(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        Ok(self.viewport.resize(size.width, size.height)?)
    }
}