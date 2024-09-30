use app::demo;
use dom::{Result, Window};
use graph::*;
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
    let vao_writer = vao.writer().attribute(att).apex()?;
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
    let index_bufferer = index_buffer.writer().array(index_array).apex()?;
    let att = buffer.attribute().size(3).node()?;
    let vao = gpu.vao()?;
    let vao_writer = vao.writer().attribute(att).index(index_buffer).apex()?;
    let elements = gpu
        .draw_elements(program)
        .stem(bufferer.clone().hub())
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
    let index_bufferer = index_buffer.writer().array(index_array).apex()?;
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
        .apex()?;
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

pub fn make_vertex_shader() -> Result<()> {
    let gpu = gpu()?;
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

pub fn make_fragment_shader() -> Result<()> {
    let gpu = gpu()?;
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT_RED) {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

pub fn make_program() -> Result<()> {
    let gpu = gpu()?;
    basic_program(&gpu)?;
    Ok(())
}

pub fn make_buffer() -> Result<Node<Bufferer>> {
    let gpu = gpu()?;
    let (_, bufferer) = make_basic_buffer(&gpu)?;
    Ok(bufferer)
}

pub fn make_index_buffer() -> Result<()> {
    let gpu = gpu()?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.buffer()?.index();
    index_buffer.writer().array(array).node()?;
    Ok(())
}

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

pub fn make_vertex_array_object() -> Result<()> {
    let gpu = gpu()?;
    let (buffer, _) = make_basic_buffer(&gpu)?;
    let att = buffer.attribute().size(3).node()?;
    gpu.vao()?.writer().attribute(att);
    Ok(())
}

pub async fn draw_arrays() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_arrays_basic(&gpu).await?;
    Ok(())
}

pub async fn draw_elements() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_elements_basic(&gpu).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
pub async fn draw_elements_react_to_shader_source() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    let (_draw, shader_source, _buff) = draw_elements_basic(&gpu).await?;
    shader_source
        .write(|source| *source = shader::basic::FRAGMENT_GREEN.to_owned())
        .await?;
    Ok(())
}

pub async fn draw_elements_react_to_buffer_array() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    let (_elements, _shader, buffer) = draw_elements_basic(&gpu).await?;
    let array: Vec<f32> = vec![0.1, 0.8, 0., 0.9, 0.8, 0., 0.9, 0., 0.];
    buffer.write(|pack| pack.unit.array(array)).await?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
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

pub async fn draw_elements_textured() -> Result<()> {
    let gpu = gpu_on_canvas()?;
    draw_elements_textured_basic(&gpu).await?;
    Ok(())
}

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
    let vao_writer = vao.writer().attribute(att).apex()?;
    let target = gpu.buffer()?;
    let sizer = target.writer().array(36).apex()?;
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
    assert_eq!(
        Vf32(vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
        reader.base().await?
    );
    Ok(())
}

pub fn nurbs() -> app::Result<()> {
    demo::nurbs::DemoBuilder::default().make()?.start();
    Ok(())
}

// pub async fn nurbs_shader() -> Result<()> {
//     let gpu = gpu()?;
//     let vertex = gpu.vertex_shader(shader::basic::NURBS)?;
//     let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_EMPTY)?;
//     let program = gpu
//         .program(vertex, fragment)?
//         .out("position0")
//         .out("position1")
//         .node()?;
//     #[rustfmt::skip]
//     let knots: Vec<f32> = vec![
//         8.,     0.,   0.,   0.,   0.,    0.,   0.,   0.,   0.,    1.,   1.,   1.,   1.,    1.,   1.,   1.,   1.,
//         6.,     0.,   0.,   0.,   0.,    0.,   0.,   0.,   0.,    0.,   0.,   1.,   1.,    1.,   1.,   1.,   1.,
//         8.,     0.,   0.,   0.,   0.,    0.,   0.,   0.,   0.,    1.,   1.,   1.,   1.,    1.,   1.,   1.,   1.,
//     ];
//     let buffer = gpu.buffer()?;
//     let bufferer = buffer.writer().array(knots).apex()?;

//     let attribs = vec![
//         buffer.attribute().size(1).stride(68).node()?,
//         buffer
//             .attribute()
//             .size(4)
//             .stride(68)
//             .offset(4)
//             .index(1)
//             .node()?,
//         buffer
//             .attribute()
//             .size(4)
//             .stride(68)
//             .offset(20)
//             .index(2)
//             .node()?,
//         buffer
//             .attribute()
//             .size(4)
//             .stride(68)
//             .offset(36)
//             .index(3)
//             .node()?,
//         buffer
//             .attribute()
//             .size(4)
//             .stride(68)
//             .offset(52)
//             .index(4)
//             .node()?,
//     ];
//     let vao = gpu.vao()?;
//     let vao_writer = vao.writer().attributes(attribs).apex()?;
//     let target = gpu.buffer()?;
//     let sizer = target.writer().array(1000).apex()?;
//     let tfo = gpu.tfo()?.buffer(&target).make()?;
//     let draw = gpu
//         .draw_arrays(program)
//         .mode(WGLRC::POINTS)
//         .stem(bufferer)
//         .stem(vao_writer)
//         .stem(sizer)
//         .vao(vao)
//         .tfo(tfo)
//         .rasterizer_discard(true)
//         .count(3)
//         .node()?;
//     target.reader().size(100).draw(draw).node()?;
//     Ok(())
// }
