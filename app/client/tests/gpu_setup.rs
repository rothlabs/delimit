use gpu::*;
use graph::*;
use std::error::Error;

pub fn make_canvas() -> Gpu {
    let canvas = Canvas::link();
    canvas.read(|unit| unit.gpu())
}

pub fn make_canvas_on_body() -> Gpu {
    let canvas = Canvas::link();
    let gpu = canvas.read(|unit| {
        unit.add_to_body();
        unit.gpu()
    });
    canvas.act();
    gpu
}

pub fn make_basic_program(gpu: &Gpu) -> (Agent<Program>, Ace<String>) {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment_source = shader::basic::FRAGMENT_RED.ace();
    let fragment = gpu.fragment_shader(&fragment_source).unwrap();
    let program = gpu.program(&vertex, &fragment);
    if let Err(memo) = program {
        panic!("gpu error: {memo}");
    }
    (program.unwrap(), fragment_source)
}

pub fn make_tex_program(gpu: &Gpu) -> program::Result {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX_TEX)?;
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_TEX)?;
    gpu.program(&vertex, &fragment)
}

pub fn make_basic_array_buffer(gpu: &Gpu) -> buffer::Result<f32> {
    #[rustfmt::skip]
    let buffer = gpu.array_buffer(vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ])?;
    Ok(buffer)
}

pub fn make_vertex_color_buffer(gpu: &Gpu) -> buffer::Result<f32> {
    #[rustfmt::skip]
    let buffer = gpu.array_buffer(vec![
        // xyz          // uv
        0.,  0.,  0.,   0., 0.,
        0., 0.8,  0.,   0., 1.,
        0.8,  0., 0.,   1., 0.,
    ])?;
    Ok(buffer)
}

pub fn make_basic_texture(gpu: &Gpu) -> texture::Result<u8> {
    #[rustfmt::skip]
    let texture = gpu.texture(vec![
        255,255,255,		230,25,75,			60,180,75,			255,225,25,
        67,99,216,			245,130,49,			145,30,180,			70,240,240,
        240,50,230,			188,246,12,			250,190,190,		0,128,128,
        230,190,255,		154,99,36,			255,250,200,		0,0,0,
    ])?.width(4).height(4).link_u8()?;
    Ok(texture)
}

pub fn draw_basic_elements(gpu: &Gpu) -> Result<(Agent<Elements>, Ace<String>), Box<dyn Error>> {
    let (program, vertex_source) = make_basic_program(&gpu);
    let buffer = make_basic_array_buffer(&gpu)?;
    let index_buffer = gpu.index_buffer(vec![0, 1, 2])?;
    let att = gpu.vertex_attribute(&buffer).size(3).link()?;
    let vao = gpu.vao(&vec![att])?.index_buffer(index_buffer).link()?;
    let elements = gpu.elements(&program, &buffer, &vao);
    elements.write(|pack| {
        pack.unit.count(3);
    })?;
    elements.act()?;
    Ok((elements, vertex_source))
}

pub fn draw_with_tex(gpu: &Gpu) -> Result<Agent<Elements>, Box<dyn Error>> {
    let program = make_tex_program(&gpu)?;
    let buffer = make_vertex_color_buffer(&gpu)?;
    let index_buffer = gpu.index_buffer(vec![0, 1, 2])?;
    let pos = gpu.vertex_attribute(&buffer).size(3).stride(20).link()?;
    let uv = gpu
        .vertex_attribute(&buffer)
        .size(2)
        .stride(20)
        .offset(12)
        .link()?;
    let vao = gpu.vao(&vec![pos, uv])?.index_buffer(index_buffer).link()?;
    let elements = gpu.elements(&program, &buffer, &vao);
    elements.write(|pack| {
        pack.unit.count(3);
    })?;
    elements.act()?;
    Ok(elements)
}