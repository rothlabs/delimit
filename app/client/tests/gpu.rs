use gpu::*;
use graph::*;
// use gpu_setup::draw_basic_elements;
// use gpu_setup::*;
use std::error::Error;

// mod gpu_setup;

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
    canvas.solve(Task::Main).ok();
    gpu
}

pub fn make_basic_program(gpu: &Gpu) -> (Node<Program>, Leaf) {
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment_source = shader::basic::FRAGMENT_RED.leaf();
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

pub fn make_basic_buffer(gpu: &Gpu) -> buffer::Result {
    #[rustfmt::skip]
    let array: Vec<f32> = vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ];
    let buffer = gpu.buffer(array)?;
    Ok(buffer)
}

pub fn make_vertex_color_buffer(gpu: &Gpu) -> buffer::Result {
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

pub fn make_basic_texture(gpu: &Gpu) -> texture::Result {
    #[rustfmt::skip]
    let array: Vec<u8> = vec![
        128,128,128,		230,25,75,			60,180,75,			255,225,25,
        67,99,216,			245,130,49,			145,30,180,			70,240,240,
        240,50,230,			188,246,12,			250,190,190,		0,128,128,
        230,190,255,		154,99,36,			255,250,200,		0,0,0,
    ];
    let texture = gpu.texture(array)?.width(4_i32).height(4_i32).link()?;
    Ok(texture)
}

pub fn draw_elements_basic(gpu: &Gpu) -> Result<(Node<Elements>, Leaf), Box<dyn Error>> {
    let (program, vertex_source) = make_basic_program(&gpu);
    let buffer = make_basic_buffer(&gpu)?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.index_buffer(array)?;
    let att = gpu.vertex_attribute(&buffer).size(3_i32).link()?;
    let vao = gpu.vao(&vec![att])?.index_buffer(index_buffer).link()?;
    let elements = gpu
        .elements(&program)
        .buffer(buffer)
        .vao(vao)
        .count(3_i32)
        .link()?;
    elements.solve(Task::Main)?;
    Ok((elements, vertex_source))
}

pub fn draw_elements_textured_basic(gpu: &Gpu) -> Result<Node<Elements>, Box<dyn Error>> {
    let program = make_tex_program(&gpu)?;
    let buffer = make_vertex_color_buffer(&gpu)?;
    let array: Vec<u16> = vec![0, 1, 2];
    let index_buffer = gpu.index_buffer(array)?;
    let pos = gpu.vertex_attribute(&buffer).size(3_i32).stride(20_i32).link()?;
    let uv = gpu
        .vertex_attribute(&buffer)
        .index(1_u32)
        .size(2_i32)
        .stride(20_i32)
        .offset(12_i32)
        .link()?;
    let vao = gpu.vao(&vec![pos, uv])?.index_buffer(index_buffer).link()?;
    let _ = make_basic_texture(&gpu)?;
    let elements = gpu
        .elements(&program)
        .buffer(buffer)
        .vao(vao)
        .count(3_i32)
        .link()?;
    elements.solve(Task::Main)?;
    Ok(elements)
}

//////////////////////////////////////
////////////////////////////////////// Tests
//////////////////////////////////////

pub fn make_vertex_shader() {
    let gpu = make_canvas();
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_fragment_shader() {
    let gpu = make_canvas();
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT_RED) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_program() {
    let gpu = make_canvas();
    make_basic_program(&gpu);
}

pub fn make_buffer() -> buffer::Result { // f32
    let gpu = make_canvas();
    make_basic_buffer(&gpu)
}

pub fn make_index_buffer() {
    let gpu = make_canvas();
    let array: Vec<u16> = vec![0, 1, 2];
    let buffer = gpu.index_buffer(array);
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
}

pub fn make_vertex_attribute() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas();
    let buffer = make_basic_buffer(&gpu)?;
    gpu.vertex_attribute(&buffer)
        .index(0_u32)
        .size(3_i32)
        .stride(0_i32)
        .offset(0_i32)
        .link()?;
    Ok(())
}

pub fn make_vertex_array_object() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas();
    let buffer = make_basic_buffer(&gpu)?;
    let att = gpu.vertex_attribute(&buffer).size(3_i32).link()?;
    gpu.vao(&vec![att])?;
    Ok(())
}

pub fn draw_elements() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas_on_body();
    draw_elements_basic(&gpu)?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
pub fn elements_react_to_shader_source() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas_on_body();
    let (_elements, shader_source) = draw_elements_basic(&gpu)?;
    shader_source.write(|tray| {
        if let Tray::String(source) = tray {
            *source = shader::basic::FRAGMENT_GREEN.to_owned()
        } else {
            panic!("not a string")
        }
    })?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
pub fn shader_source_error() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas();
    let (_elements, shader_source) = draw_elements_basic(&gpu)?;
    if let Err(_) = shader_source.write(|tray| 
        if let Tray::String(string) = tray {
            *string = "bad shader".to_owned()
        } else {
            panic!("not a string")
        }
    ) {
        Ok(())
    } else {
        panic!("this shader write should have caused compile error");
    }
}

pub fn draw_elements_textured() -> Result<(), Box<dyn Error>> {
    let gpu = make_canvas_on_body();
    draw_elements_textured_basic(&gpu)?;
    Ok(())
}
