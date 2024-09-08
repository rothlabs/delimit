use super::*;
use web_sys::WebGlTexture;

// type BuilderResult = std::result::Result<Node<Texture>, TextureBuilderError>;

#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct Texture {
    gl: WGLRC,
    texture: WebGlTexture,
    /// Linear data array of image.
    array: Apex,
    /// Horizontal pixel count.
    #[builder(default)]
    width: Hub<i32>,
    /// Vertical pixel count.
    #[builder(default)]
    height: Hub<i32>,
}

impl TextureBuilder {
    pub fn make(&self) -> Result<Node<Texture>> {
        let mut texture = self.build().map_err(|err| anyhow!("{}", err.to_string()))?;
        let node = Node::make(|back| {
            texture.array = texture.array.backed(back)?;
            texture.width = texture.width.backed(back)?;
            texture.height = texture.height.backed(back)?;
            Ok(texture)
        })?;
        node.act()?;
        Ok(node)
    }
}

impl Texture {
    pub fn bind(&self) {
        // self.gl.active_texture(WGLRC::TEXTURE0);
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&self.texture));
    }
    fn vec_u8(&self, array: &Vec<u8>) -> Result<()> {
        let pixels = unsafe { Uint8Array::view(array.as_slice()) };
        // TODO: use PIXEL_UNPACK_ buffer bind and following pbo offset:
        // self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(target, level, internalformat, width, height, border, format, type_, pbo_offset)
        if let Err(memo) = self
            .gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WGLRC::TEXTURE_2D,                      // target
                0,                                      // level,
                WGLRC::RGB as i32,                      // internalformat,
                self.width.base().unwrap_or_default(),  // width
                self.height.base().unwrap_or_default(), // height
                0,                                      // border,
                WGLRC::RGB,                             // format
                WGLRC::UNSIGNED_BYTE,                   // type_
                Some(&pixels),                          // pixels
            )
        {
            let memo = memo
                .as_string()
                .unwrap_or("unknown error in texture".into());
            Err(anyhow!(memo))?
        }
        Ok(())
    }
}

impl Act for Texture {
    fn act(&self) -> Result<()> {
        self.bind();
        match &self.array {
            Apex::Vu8(array) => array.read(|array| self.vec_u8(array))?,
            _ => Err(anyhow!("wrong apex"))?,
        }
    }
}

impl Adapt for Texture {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}

// self.gl.tex_storage_2d(target, levels, internalformat, width, height)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
