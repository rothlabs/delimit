use gpu::*;
use graph::*;
use texture::Texture;
use wasm_bindgen_test::console_log;

pub fn make_canvas() -> Result<Gpu> {
    let canvas = Canvas::new();
    canvas.read(|unit| unit.gpu())
}

pub async fn make_canvas_on_body() -> Result<Gpu> {
    let canvas = Canvas::new();
    let gpu = canvas.read(|unit| {
        unit.add_to_body();
        unit.gpu()
    })?;
    canvas.act().await?;
    Ok(gpu)
}

pub fn make_basic_program(gpu: &Gpu) -> Result<(Node<Program>, Leaf<String>)> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment_source = shader::basic::FRAGMENT_RED.leaf();
    let fragment = gpu.fragment_shader(&fragment_source).unwrap();
    let program = gpu.program(&vertex, &fragment)?.make()?;
    Ok((program, fragment_source))
}

pub fn make_tex_program(gpu: &Gpu) -> Result<Node<Program>> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_TEX)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_TEX)?;
    gpu.program(&vertex, &fragment)?.make()
}

pub fn make_basic_buffer(gpu: &Gpu) -> Result<(Buffer, Node<Bufferer>)> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ];
    let buffer = gpu.buffer()?;
    let bufferer = gpu.bufferer(&buffer).array(array).make()?;
    Ok((buffer, bufferer))
}

pub fn make_vertex_color_buffer(gpu: &Gpu) -> Result<(Buffer, Node<Bufferer>)> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        // xyz             // uv
        -0.8, 0.,  0.,     0., 0.,
        0.,   0.8, 0.,     0.5, 1.,
        0.8,  0.,  0.,     1., 0.,
    ];
    let buffer = gpu.buffer()?;
    let bufferer = gpu.bufferer(&buffer).array(array).make()?;
    Ok((buffer, bufferer))
}

pub async fn make_basic_texture(gpu: &Gpu) -> Result<Node<Texture>> {
    #[rustfmt::skip]
    let array: Vec<u8> = vec![
        128,128,128,		230,25,75,			60,180,75,			255,225,25,
        67,99,216,			245,130,49,			145,30,180,			70,240,240,
        240,50,230,			188,246,12,			250,190,190,		0,128,128,
        230,190,255,		154,99,36,			255,250,200,		0,0,0,
    ];
    let texture = gpu.texture(array)?.width(4).height(4).make()?;
    texture.act().await?;
    Ok(texture)
}

pub async fn draw_arrays_basic(gpu: &Gpu) -> Result<(Node<DrawArrays>, Node<Bufferer>)> {
    let (program, _) = make_basic_program(&gpu)?;
    let (buffer, bufferer) = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    let vao = gpu.vao(vec![att])?.make()?;
    let draw_arrays = gpu
        .draw_arrays(program)
        .bufferers(vec![bufferer.clone()])
        .vao(vao)
        .count(3)
        .make()?;
    draw_arrays.act().await?;
    Ok((draw_arrays, bufferer))
}

pub async fn draw_elements_basic(gpu: &Gpu) -> Result<(Node<DrawElements>, Leaf<String>, Node<Bufferer>)> {
    let (program, vertex_source) = make_basic_program(&gpu)?;
    let (buffer, bufferer) = make_basic_buffer(&gpu)?;
    let index_array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    let index_bufferer = gpu.bufferer(&index_buffer).array(index_array).make()?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    let vao = gpu.vao(vec![att])?.index_buffer(index_buffer).make()?;
    let elements = gpu
        .draw_elements(program)
        .buffers(vec![bufferer.clone(), index_bufferer])
        .vao(vao)
        .count(3)
        .make()?;
    elements.act().await?;
    Ok((elements, vertex_source, bufferer))
}

pub async fn draw_elements_textured_basic(gpu: &Gpu) -> Result<Node<DrawElements>> {
    let program = make_tex_program(&gpu)?;
    let (buffer, bufferer) = make_vertex_color_buffer(&gpu)?;
    let index_array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    let index_bufferer = gpu.bufferer(&index_buffer).array(index_array).make()?;
    let pos = gpu.vertex_attribute(&buffer).size(3).stride(20).make()?;
    let uv = gpu
        .vertex_attribute(&buffer)
        .index(1)
        .size(2)
        .stride(20)
        .offset(12)
        .make()?;
    let vao = gpu.vao(vec![pos, uv])?.index_buffer(index_buffer).make()?;
    let _ = make_basic_texture(&gpu).await?;
    let elements = gpu
        .draw_elements(program)
        .buffers(vec![bufferer, index_bufferer])
        .vao(vao)
        .count(3)
        .make()?;
    elements.act().await?;
    Ok(elements)
}

//////////////////////////////////////
////////////////////////////////////// Tests
//////////////////////////////////////

pub fn make_vertex_shader() -> Result<()> {
    let gpu = make_canvas()?;
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

pub fn make_fragment_shader() -> Result<()> {
    let gpu = make_canvas()?;
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT_RED) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

pub fn make_program() -> Result<()> {
    let gpu = make_canvas()?;
    make_basic_program(&gpu)?;
    Ok(())
}

pub fn make_buffer() -> Result<Node<Bufferer>> {
    let gpu = make_canvas()?;
    let (_, bufferer) = make_basic_buffer(&gpu)?;
    Ok(bufferer)
}

pub fn make_index_buffer() -> Result<()> {
    let gpu = make_canvas()?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    let _ = gpu.bufferer(&index_buffer).array(array).make()?;
    Ok(())
}

pub fn make_vertex_attribute() -> Result<()> {
    let gpu = make_canvas()?;
    let (buffer, _) = make_basic_buffer(&gpu)?;
    gpu.vertex_attribute(buffer)
        .index(0)
        .size(3)
        // .stride(0)
        // .offset(0)
        .make()?;
    Ok(())
}

pub fn make_vertex_array_object() -> Result<()> {
    let gpu = make_canvas()?;
    let (buffer, _) = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    gpu.vao(vec![att])?;
    Ok(())
}

pub async fn draw_arrays() -> Result<()> {
    let gpu = make_canvas_on_body().await?;
    draw_arrays_basic(&gpu).await?;
    Ok(())
}

pub async fn draw_elements() -> Result<()> {
    let gpu = make_canvas_on_body().await?;
    draw_elements_basic(&gpu).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
pub async fn draw_elements_react_to_shader_source() -> Result<()> {
    let gpu = make_canvas_on_body().await?;
    let (_draw, shader_source, _buff) = draw_elements_basic(&gpu).await?;
    shader_source.write(|source| *source = shader::basic::FRAGMENT_GREEN.to_owned()).await?;
    Ok(())
}

pub async fn draw_elements_react_to_buffer_array() -> Result<()> {
    let gpu = make_canvas_on_body().await?;
    let (_elements, _shader, buffer) = draw_elements_basic(&gpu).await?;
    let array: Vec<f32> = vec![
        0.1,  0.8,  0.,
        0.9, 0.8,  0.,
        0.9,  0., 0.,
    ];
    buffer.write(|pack| pack.unit.array(array)).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
pub async fn shader_source_error() -> Result<()> {
    let gpu = make_canvas()?;
    let (_elements, shader_source, _buff) = draw_elements_basic(&gpu).await?;
    if let Err(_) = shader_source.write(|source| *source = "bad shader".to_owned()).await {
        Ok(())
    } else {
        panic!("this shader write should have caused compile error");
    }
}

pub async fn draw_elements_textured() -> Result<()> {
    let gpu = make_canvas_on_body().await?;
    draw_elements_textured_basic(&gpu).await?;
    Ok(())
}

pub async fn transform_feedback() -> Result<()> {
    let gpu = make_canvas()?;
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_FEEDBACK)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_EMPTY)?;
    let program = gpu.program(&vertex, &fragment)?.outs(vec!["output0".into(), "output1".into()]).make()?;
    let (buffer, bufferer) = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    let vao = gpu.vao(vec![att])?.make()?;

    let buffer = gpu.buffer()?;
    let buffer_sizer = gpu.bufferer(&buffer).array(36).make()?;
    buffer_sizer.act().await?;
    let tfo = gpu.tfo(vec![buffer.clone()])?;

    let draw_arrays = gpu
        .draw_arrays(program)
        .bufferers(vec![bufferer.clone()])
        .vao(vao)
        .tfo(tfo)
        .rasterizer_discard(true)
        .count(3)
        .make()?;
    draw_arrays.act().await?;

    let buffer_in = gpu.buffer_in(&buffer).size(6).draw(draw_arrays).make()?;
    assert_eq!(Vf32(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0]), buffer_in.base().await?);
    Ok(())
}
