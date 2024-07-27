use super::*;


pub type Result<T> = std::result::Result<Agent<Texture<T>>, Box<dyn Error>>;

#[derive(Builder)]
#[builder(setter(into))]
pub struct Texture<T> {
    gl: WGLRC,
    texture: WebGlTexture,
    /// Linear data array of image.
    array: Array<T>,
    /// Horizontal pixel count.
    #[builder(default)]
    width: Value<i32>,
    /// Vertical pixel count.
    #[builder(default)]
    height: Value<i32>,
}

impl<T> Texture<T> {
    pub fn bind(&self) {
        // self.gl.active_texture(WGLRC::TEXTURE0);
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&self.texture));
    }
}

impl TextureBuilder<u8> {
    pub fn link_u8(&self) -> Result<u8> {
        let mut texture = self.build()?;
        let link = Agent::make(|back| {
            texture.array = texture.array.backed(back);
            texture.width = texture.width.backed(back);
            texture.height = texture.height.backed(back);
            texture
        });
        link.act()?;
        Ok(link)
    }
}

impl Act for Texture<u8> {
    type Load = react::Result;
    fn act(&self) -> Self::Load {
        self.bind();
        self.array.read(|unit| {
            let pixels = unsafe {
                Uint8Array::view(unit.as_slice())
            };
            // TODO: use PIXEL_UNPACK_ buffer bind and following pbo offset:
            // self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_i32(target, level, internalformat, width, height, border, format, type_, pbo_offset)
            if let Err(memo) = self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
                WGLRC::TEXTURE_2D, // target
                0, // level, 
                WGLRC::RGB as i32, // internalformat, 
                self.width.load(), // width 
                self.height.load(), // height
                0, // border, 
                WGLRC::RGB, // format
                WGLRC::UNSIGNED_BYTE, // type_
                Some(&pixels), // pixels
            ) {
                return Err(memo.as_string().unwrap());
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl React for Texture<u8> {
    fn react(&self, _: &Meta) -> react::Result {
        self.act()
    }
}

// impl React for Texture<f32> {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act();
//         Ok(())
//     }
// }

// self.gl.tex_storage_2d(target, levels, internalformat, width, height)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(target, level, internalformat, width, height, border, format, type_, pixels)
// self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_u8_array_and_src_offset(target, level, internalformat, width, height, border, format, type_, src_data, src_offset)

// impl Act for Texture<f32> {
//     type Load = ();
//     fn act(&self) -> Self::Load {
//         self.bind();
//         self.array.grant().read(|unit| unsafe {
//         });
//         self.unbind();
//     }
// }

// impl Texture<f32> {
//     pub fn link_f32(gl: &WGLRC, target: u32, array: &Array<f32>) -> Result<f32> {
//         let texture = gl.create_texture().ok_or("failed to create texture")?;
//         let link = Agent::make(|back| Self {
//             gl: gl.clone(),
//             target,
//             texture,
//             array: array.backed(back),
//         });
//         link.act();
//         Ok(link)
//     }
// }

// pub fn size(
//     &mut self,
//     width: impl Into<Value<i32>>,
//     height: impl Into<Value<i32>>,
// ) -> &mut Self {
//     self.width = width.into();
//     self.height = height.into();
//     self
// }

// impl Texture<u8> {
//     pub fn link_u8(gl: &WGLRC, array: &Array<u8>) -> Result<u8> {
//         let texture = gl.create_texture().ok_or("failed to create texture")?;
//         gl.bind_texture(WGLRC::TEXTURE_2D, Some(&texture));
//         gl.tex_parameteri(
//             WGLRC::TEXTURE_2D,
//             WGLRC::TEXTURE_MIN_FILTER,
//             WGLRC::NEAREST as i32,
//         );
//         gl.tex_parameteri(
//             WGLRC::TEXTURE_2D,
//             WGLRC::TEXTURE_MAG_FILTER,
//             WGLRC::NEAREST as i32,
//         );
//         let link = Agent::make(|back| Self {
//             gl: gl.clone(),
//             texture,
//             array: array.backed(back),
//             width: Value::default(),
//             height: Value::default(),
//         });
//         link.act()?;
//         Ok(link)
//     }
// }
