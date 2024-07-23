use super::*;


pub struct VertexAttribute {
    gl: WGLRC,
    index: AceView<u32>,
    size: AceView<i32>,
    stride: AceView<i32>,
    offset: AceView<i32>,
}

impl VertexAttribute {
    pub fn link(wglrc: &WGLRC) -> Agent<VertexAttribute> {
        Agent::new(Self {
            gl: wglrc.clone(),
            index: AceView::default(),
            size: AceView::default(),
            stride: AceView::default(),
            offset: AceView::default(),
        })
    }
    pub fn index(&mut self, index: impl Into<AceView<u32>>) -> &mut Self {
        self.index = index.into();
        self
    }
    pub fn size(&mut self, size: impl Into<AceView<i32>>) -> &mut Self {
        self.size = size.into();
        self
    }
    pub fn stride(&mut self, stride: impl Into<AceView<i32>>) -> &mut Self {
        self.stride = stride.into();
        self
    }
    pub fn offset(&mut self, offset: impl Into<AceView<i32>>) -> &mut Self {
        self.offset = offset.into();
        self
    }
}

impl Act for VertexAttribute {
    type Load = ();
    fn act(&self) -> Self::Load {
        let index = self.index.load();
        let size = self.size.load();
        let stride = self.stride.load();
        let offset = self.offset.load();
        self.gl.vertex_attrib_pointer_with_i32(index, size, WGLRC::FLOAT, false, stride, offset);
        self.gl.enable_vertex_attrib_array(index);
    }
}

// self.gl.vertex_attrib_pointer_with_f64(indx, size, type_, normalized, stride, offset)