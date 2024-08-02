use super::*;
use web_sys::WebGlTexture;

pub type Result = std::result::Result<Agent<Texture>, graph::Error>;

#[derive(Builder)]
#[builder(setter(into))]
pub struct Texture {
    gl: WGLRC,
    texture: WebGlTexture,
    /// Linear data array of image.
    array: Node,
    /// Horizontal pixel count.
    #[builder(default)]
    width: Node,
    /// Vertical pixel count.
    #[builder(default)]
    height: Node,
}

impl Texture {
    pub fn bind(&self) {
        // self.gl.active_texture(WGLRC::TEXTURE0);
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&self.texture));
    }
}

impl TextureBuilder {
    pub fn link(&self) -> Result {
        let mut texture = self.build()?;
        let link = Agent::make(|back| {
            texture.array = texture.array.backed(back);
            texture.width = texture.width.backed(back);
            texture.height = texture.height.backed(back);
            texture
        });
        link.solve(Task::None)?;
        Ok(link)
    }
}

impl Solve for Texture {
    fn solve(&self, _: Task) -> solve::Result {
        self.bind();
        self.array.read_vu8(|unit| {
            let pixels = unsafe {
                Uint8Array::view(unit.as_slice())
            };
            // TODO: use PIXEL_UNPACK_ buffer bind and following pbo offset:
            // self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(target, level, internalformat, width, height, border, format, type_, pbo_offset)
            if let Err(memo) = self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WGLRC::TEXTURE_2D, // target
                0, // level, 
                WGLRC::RGB as i32, // internalformat, 
                self.width.i32(), // width 
                self.height.i32(), // height
                0, // border, 
                WGLRC::RGB, // format
                WGLRC::UNSIGNED_BYTE, // type_
                Some(&pixels), // pixels
            ) {
                return Err(memo.as_string().unwrap());
            }
            Ok(())
        })?;
        Ok(Tray::None)
    }
}

// self.gl.tex_storage_2d(target, levels, internalformat, width, height)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)