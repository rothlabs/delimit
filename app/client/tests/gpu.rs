use gpu::*;
use graph::*;
use texture::Texture;

pub fn make_canvas() -> Result<Gpu> {
    let canvas = Canvas::new();
    canvas.read(|unit| unit.gpu())
}

pub fn make_canvas_on_body() -> Result<Gpu> {
    let canvas = Canvas::new();
    let gpu = canvas.read(|unit| {
        unit.add_to_body();
        unit.gpu()
    })?;
    canvas.solve(Task::Main).ok();
    Ok(gpu)
}

pub fn make_basic_program(gpu: &Gpu) -> Result<(Node<Program>, Leaf<String>)> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment_source = shader::basic::FRAGMENT_RED.leaf();
    let fragment = gpu.fragment_shader(&fragment_source).unwrap();
    let program = gpu.program(&vertex, &fragment)?.make();
    if let Err(memo) = program {
        panic!("gpu error: {memo}");
    }
    Ok((program.unwrap(), fragment_source))
}

pub fn make_tex_program(gpu: &Gpu) -> Result<Node<Program>> {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_TEX)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_TEX)?;
    gpu.program(&vertex, &fragment)?.make()
}

pub fn make_basic_buffer(gpu: &Gpu) -> Result<Node<Buffer>> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ];
    let buffer = gpu.buffer(array)?;
    Ok(buffer)
}

pub fn make_vertex_color_buffer(gpu: &Gpu) -> Result<Node<Buffer>> {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        // xyz             // uv
        -0.8, 0.,  0.,     0., 0.,
        0.,   0.8, 0.,     0.5, 1.,
        0.8,  0.,  0.,     1., 0.,
    ];
    let buffer = gpu.buffer(array)?;
    Ok(buffer)
}

pub fn make_basic_texture(gpu: &Gpu) -> Result<Node<Texture>> {
    #[rustfmt::skip]
    let array: Vec<u8> = vec![
        128,128,128,		230,25,75,			60,180,75,			255,225,25,
        67,99,216,			245,130,49,			145,30,180,			70,240,240,
        240,50,230,			188,246,12,			250,190,190,		0,128,128,
        230,190,255,		154,99,36,			255,250,200,		0,0,0,
    ];
    let texture = gpu.texture(array)?.width(4).height(4).make()?;
    Ok(texture)
}

pub fn draw_arrays_basic(gpu: &Gpu) -> Result<(Node<DrawArrays>, Node<Buffer>)> {
    let (program, _) = make_basic_program(&gpu)?;
    let buffer = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    let vao = gpu.vao(&vec![att])?.make()?;
    let draw_arrays = gpu
        .draw_arrays(&program)
        .vao(vao)
        .count(3)
        .make()?;
    draw_arrays.act()?;
    Ok((draw_arrays, buffer))
}

pub fn draw_elements_basic(gpu: &Gpu) -> Result<(Node<DrawElements>, Leaf<String>, Node<Buffer>)> {
    let (program, vertex_source) = make_basic_program(&gpu)?;
    let buffer = make_basic_buffer(&gpu)?;
    let index_array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.index_buffer(index_array)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    let vao = gpu.vao(&vec![att])?.index_buffer(index_buffer).make()?;
    let elements = gpu
        .draw_elements(&program)
        .vao(vao)
        .count(3)
        .make()?;
    elements.act()?;
    Ok((elements, vertex_source, buffer))
}

pub fn draw_elements_textured_basic(gpu: &Gpu) -> Result<Node<DrawElements>> {
    let program = make_tex_program(&gpu)?;
    let buffer = make_vertex_color_buffer(&gpu)?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.index_buffer(array)?;
    let pos = gpu.vertex_attribute(&buffer).size(3).stride(20).make()?;
    let uv = gpu
        .vertex_attribute(&buffer)
        .index(1)
        .size(2)
        .stride(20)
        .offset(12)
        .make()?;
    let vao = gpu.vao(&vec![pos, uv])?.index_buffer(index_buffer).make()?;
    let _ = make_basic_texture(&gpu)?;
    let elements = gpu
        .draw_elements(&program)
        .vao(vao)
        .count(3)
        .make()?;
    elements.solve(Task::Main)?;
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

pub fn make_buffer() -> Result<Node<Buffer>> {
    let gpu = make_canvas()?;
    let buffer = make_basic_buffer(&gpu)?;
    Ok(buffer)
}

pub fn make_index_buffer() -> Result<()> {
    let gpu = make_canvas()?;
    let array: Vec<u16> = vec![0, 1, 2];
    let buffer = gpu.index_buffer(array);
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
    Ok(())
}

pub fn make_vertex_attribute() -> Result<()> {
    let gpu = make_canvas()?;
    let buffer = make_basic_buffer(&gpu)?;
    gpu.vertex_attribute(&buffer)
        .index(0)
        .size(3)
        // .stride(0)
        // .offset(0)
        .make()?;
    Ok(())
}

pub fn make_vertex_array_object() -> Result<()> {
    let gpu = make_canvas()?;
    let buffer = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3).make()?;
    gpu.vao(&vec![att])?;
    Ok(())
}

pub fn draw_arrays() -> Result<()> {
    let gpu = make_canvas_on_body()?;
    draw_arrays_basic(&gpu)?;
    Ok(())
}

pub fn draw_elements() -> Result<()> {
    let gpu = make_canvas_on_body()?;
    draw_elements_basic(&gpu)?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
pub fn draw_elements_react_to_shader_source() -> Result<()> {
    let gpu = make_canvas_on_body()?;
    let (_draw, shader_source, _buff) = draw_elements_basic(&gpu)?;
    shader_source.write(|source| *source = shader::basic::FRAGMENT_GREEN.to_owned())?;
    Ok(())
}

pub fn draw_elements_react_to_buffer_array() -> Result<()> {
    let gpu = make_canvas_on_body()?;
    let (_elements, _shader, buffer) = draw_elements_basic(&gpu)?;
    buffer.write(|pack| pack.unit.array(vec![
        0.1,  0.8,  0.,
        0.9, 0.8,  0.,
        0.9,  0., 0.,
    ]))?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
pub fn shader_source_error() -> Result<()> {
    let gpu = make_canvas()?;
    let (_elements, shader_source, _buff) = draw_elements_basic(&gpu)?;
    if let Err(_) = shader_source.write(|source| *source = "bad shader".to_owned()) {
        Ok(())
    } else {
        panic!("this shader write should have caused compile error");
    }
}

pub fn draw_elements_textured() -> Result<()> {
    let gpu = make_canvas_on_body()?;
    draw_elements_textured_basic(&gpu)?;
    Ok(())
}
