use super::*;
use web_sys::WebGlTexture;

#[derive(Builder, Debug, Unit!)]
#[builder(pattern = "owned", setter(into))]
pub struct Texture {
    gl: WGLRC,
    object: WebGlTexture,
    /// Linear data array of image.
    array: Apex,
    /// Horizontal pixel count.
    #[builder(default)]
    width: Hub<i32>,
    /// Vertical pixel count.
    #[builder(default)]
    height: Hub<i32>,
}

impl Texture {
    pub fn bind(&self) {
        // self.gl.active_texture(WGLRC::TEXTURE0);
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&self.object));
    }
    fn vec_u8(&self, array: &Vec<u8>, width: i32, height: i32) -> Result<()> {
        let pixels = unsafe { Uint8Array::view(array.as_slice()) };
        // TODO: use PIXEL_UNPACK_ buffer bind and following pbo offset:
        // self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(target, level, internalformat, width, height, border, format, type_, pbo_offset)
        if let Err(memo) = self
            .gl
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WGLRC::TEXTURE_2D, // target
                0,                 // level,
                WGLRC::RGB as i32, // internalformat,
                width,
                height,
                0,                    // border,
                WGLRC::RGB,           // format
                WGLRC::UNSIGNED_BYTE, // type_
                Some(&pixels),        // pixels
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
    async fn act(&self) -> Result<()> {
        let width = self.width.base().await.unwrap_or_default();
        let height = self.height.base().await.unwrap_or_default();
        self.bind();
        match &self.array {
            Apex::Vu8(array) => {
                array
                    .read(|array| self.vec_u8(array, width, height))
                    .await?
            }
            _ => Err(anyhow!("wrong apex"))?,
        }
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.array.back(back)?;
        self.width.back(back)?;
        self.height.back(back)
    }
}

// self.gl.tex_storage_2d(target, levels, internalformat, width, height)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
