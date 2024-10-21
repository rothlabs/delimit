//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::Result;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use dom::Window;
use graph::*;
use webgl::Buffer;
use webgl::*;

pub fn gpu() -> Result<WebGl> {
    Window::new()?
        .document()?
        .body()?
        .element("canvas")?
        .canvas()?
        .webgl()
}

pub fn gpu_on_canvas() -> Result<WebGl> {
    Window::new()?
        .document()?
        .body()?
        .stem("canvas")?
        .canvas()?
        .webgl()
}

pub fn basic_program(gpu: &WebGl) -> Result<(Node<Program>, Leaf<String>)> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment_source = shader::basic::FRAGMENT_RED.leaf();
    let fragment = gpu.fragment_shader(&fragment_source).unwrap();
    let program = gpu.program(vertex, fragment)?.node()?;
    Ok((program, fragment_source))
}

pub fn make_tex_program(gpu: &WebGl) -> graph::Result<Node<Program>> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_TEX)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_TEX)?;
    gpu.program(vertex, fragment)?.node()
}

pub fn make_basic_buffer(gpu: &WebGl) -> Result<(Buffer, Node<Bufferer>)> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ];
    let buffer = gpu.buffer()?;
    let bufferer = buffer.writer().array(array).node()?;
    Ok((buffer, bufferer))
}

pub fn make_vertex_color_buffer(gpu: &WebGl) -> Result<(Buffer, Node<Bufferer>)> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        // xyz             // uv
        -0.8, 0.,  0.,     0., 0.,
        0.,   0.8, 0.,     0.5, 1.,
        0.8,  0.,  0.,     1., 0.,
    ];
    let buffer = gpu.buffer()?;
    let bufferer = buffer.writer().array(array).node()?;
    Ok((buffer, bufferer))
}

pub async fn make_basic_texture(gpu: &WebGl) -> Result<Node<Texture>> {
    #[rustfmt::skip]
    let array: Vec<u8> = vec![
        128,128,128,		230,25,75,			60,180,75,			255,225,25,
        67,99,216,			245,130,49,			145,30,180,			70,240,240,
        240,50,230,			188,246,12,			250,190,190,		0,128,128,
        230,190,255,		154,99,36,			255,250,200,		0,0,0,
    ];
    let texture = gpu.texture(array)?.width(4).height(4).node()?;
    texture.act().await?;
    Ok(texture)
}

pub async fn draw_arrays_basic(gpu: &WebGl) -> Result<()> {
    let (program, _) = basic_program(&gpu)?;
    let (buffer, buffer_writer) = make_basic_buffer(&gpu)?;
    let att = buffer.attribute().size(3).node()?;
    let vao = gpu.vao()?;
    let vao_writer = vao.writer().attribute(att).hub()?;
    let draw_arrays = gpu
        .draw_arrays(program)
        .stem(buffer_writer.hub())
        .stem(vao_writer)
        .vao(vao)
        .count(3)
        .node()?;
    draw_arrays.act().await?;
    Ok(())
}

pub async fn draw_elements_basic(
    gpu: &WebGl,
) -> Result<(Node<DrawElements>, Leaf<String>, Node<Bufferer>)> {
    let (program, vertex_source) = basic_program(&gpu)?;
    let (buffer, bufferer) = make_basic_buffer(&gpu)?;
    let index_array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    let index_bufferer = index_buffer.writer().array(index_array).hub()?;
    let att = buffer.attribute().size(3).node()?;
    let vao = gpu.vao()?;
    let vao_writer = vao.writer().attribute(att).index(index_buffer).hub()?;
    let elements = gpu
        .draw_elements(program)
        .stem(bufferer.hub())
        .stem(index_bufferer)
        .stem(vao_writer)
        .vao(vao)
        .count(3)
        .node()?;
    elements.act().await?;
    Ok((elements, vertex_source, bufferer))
}

pub async fn draw_elements_textured_basic(gpu: &WebGl) -> Result<Node<DrawElements>> {
    let program = make_tex_program(&gpu)?;
    let (buffer, bufferer) = make_vertex_color_buffer(&gpu)?;
    let index_array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    let index_bufferer = index_buffer.writer().array(index_array).hub()?;
    let pos = buffer.attribute().size(3).stride(20).node()?;
    let uv = buffer
        .attribute()
        .index(1)
        .size(2)
        .stride(20)
        .offset(12)
        .node()?;
    let vao = gpu.vao()?;
    let vao_writer = vao
        .writer()
        .attributes(vec![pos, uv])
        .index(index_buffer)
        .hub()?;
    let _texture = make_basic_texture(&gpu).await?;
    let elements = gpu
        .draw_elements(program)
        .stem(bufferer.hub())
        .stem(index_bufferer)
        .stem(vao_writer)
        .vao(vao)
        .count(3)
        .node()?;
    elements.act().await?;
    Ok(elements)
}

//////////////////////////////////////
////////////////////////////////////// Tests
//////////////////////////////////////

#[wasm_bindgen_test]
pub fn make_vertex_shader() -> Result<()> {
    let gpu = gpu()?;
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_fragment_shader() -> Result<()> {
    let gpu = gpu()?;
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT_RED) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_program() -> Result<()> {
    let gpu = gpu()?;
    basic_program(&gpu)?;
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_buffer() -> Result<()> {
    let gpu = gpu()?;
    let (_, _) = make_basic_buffer(&gpu)?;
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_index_buffer() -> Result<()> {
    let gpu = gpu()?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    index_buffer.writer().array(array).node()?;
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_vertex_attribute() -> Result<()> {
    let gpu = gpu()?;
    let (buffer, _) = make_basic_buffer(&gpu)?;
    buffer
        .attribute()
        .index(0)
        .size(3)
        // .stride(0)
        // .offset(0)
        .node()?;
    Ok(())
}

#[wasm_bindgen_test]
pub fn make_vertex_array_object() -> Result<()> {
    let gpu = gpu()?;
    let (buffer, _) = make_basic_buffer(&gpu)?;
    let att = buffer.attribute().size(3).node()?;
    gpu.vao()?.writer().attribute(att);
    Ok(())
}

#[wasm_bindgen_test]
pub async fn draw_arrays() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_arrays_basic(&gpu).await?;
    Ok(())
}

#[wasm_bindgen_test]
pub async fn draw_elements() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_elements_basic(&gpu).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
#[wasm_bindgen_test]
pub async fn draw_elements_react_to_shader_source() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    let (_draw, shader_source, _buff) = draw_elements_basic(&gpu).await?;
    shader_source
        .write(|source| *source = shader::basic::FRAGMENT_GREEN.to_owned())
        .await?;
    Ok(())
}

#[wasm_bindgen_test]
pub async fn draw_elements_react_to_buffer_array() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    let (_elements, _shader, buffer) = draw_elements_basic(&gpu).await?;
    let array: Vec<f32> = vec![0.1, 0.8, 0., 0.9, 0.8, 0., 0.9, 0., 0.];
    buffer.write(|pack| pack.unit.array(array)).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
#[wasm_bindgen_test]
pub async fn shader_source_error() -> Result<()> {
    let gpu = gpu()?;
    let (_elements, shader_source, _buff) = draw_elements_basic(&gpu).await?;
    if let Err(_) = shader_source
        .write(|source| *source = "bad shader".to_owned())
        .await
    {
        Ok(())
    } else {
        panic!("this shader write should have caused compile error");
    }
}

#[wasm_bindgen_test]
pub async fn draw_elements_textured() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_elements_textured_basic(&gpu).await?;
    Ok(())
}

#[wasm_bindgen_test]
pub async fn transform_feedback() -> Result<()> {
    let gpu = gpu()?;
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_FEEDBACK)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_EMPTY)?;
    let program = gpu
        .program(vertex, fragment)?
        .out("output0")
        .out("output1")
        .node()?;
    let (buffer, bufferer) = make_basic_buffer(&gpu)?;
    let att = buffer.attribute().size(3).node()?;
    let vao = gpu.vao()?;
    let vao_writer = vao.writer().attribute(att).hub()?;
    let target = gpu.buffer()?;
    let sizer = target.writer().array(36).hub()?;
    let tfo = gpu.tfo()?.buffer(&target).make()?;
    let draw = gpu
        .draw_arrays(program)
        .stem(bufferer.hub())
        .stem(sizer)
        .stem(vao_writer)
        .vao(vao)
        .tfo(tfo)
        .rasterizer_discard(true)
        .count(3)
        .node()?;
    let reader = target.reader().size(6).draw(draw).hub()?;
    assert_eq!(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0], reader.base().await?);
    Ok(())
}

// #[wasm_bindgen_test]
// fn make_vertex_shader() -> Result<()> {
//     gpu::make_vertex_shader()
// }

// #[wasm_bindgen_test]
// fn make_fragment_shader() -> Result<()> {
//     gpu::make_fragment_shader()
// }

// #[wasm_bindgen_test]
// fn make_program() -> Result<()> {
//     gpu::make_program()
// }

// #[wasm_bindgen_test]
// fn make_buffer() -> Result<()> {
//     gpu::make_buffer()?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// fn make_index_buffer() -> Result<()> {
//     gpu::make_index_buffer()
// }

// #[wasm_bindgen_test]
// fn make_vertex_attribute() -> Result<()> {
//     gpu::make_vertex_attribute()?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// fn make_vertex_array_object() -> Result<()> {
//     gpu::make_vertex_array_object()?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn draw_arrays() -> Result<()> {
//     gpu::draw_arrays().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn draw_elements() -> Result<()> {
//     gpu::draw_elements().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn draw_elements_react_to_shader_source() -> Result<()> {
//     gpu::draw_elements_react_to_shader_source().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn draw_elements_react_to_buffer_array() -> Result<()> {
//     gpu::draw_elements_react_to_buffer_array().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn shader_source_error() -> Result<()> {
//     gpu::shader_source_error().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn draw_elements_textured() -> Result<()> {
//     gpu::draw_elements_textured().await?;
//     Ok(())
// }

// #[wasm_bindgen_test]
// async fn transform_feedback() -> Result<()> {
//     gpu::transform_feedback().await?;
//     Ok(())
// }
