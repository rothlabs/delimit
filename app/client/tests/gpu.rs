use gpu::*;
use graph::*;

// Setup

fn make_canvas() -> Gpu {
    let canvas = Canvas::link();
    canvas.read(|unit| unit.gpu())
}

fn make_canvas_on_body() -> Gpu {
    let canvas = Canvas::link();
    let gpu = canvas.read(|unit| {
        unit.add_to_body();
        unit.gpu()
    });
    canvas.act();
    gpu
}

fn make_basic_program(gpu: &Gpu) -> (Agent<Program>, Ace<String>) {
    let vertex_source = shader::basic::VERTEX.ace();
    let vertex = gpu.vertex_shader(&vertex_source).unwrap();
    let fragment_source = shader::basic::FRAGMENT.ace();
    let fragment = gpu.fragment_shader(&fragment_source).unwrap();
    let program = gpu.program(&vertex, &fragment);
    if let Err(memo) = program {
        panic!("gpu error: {memo}");
    }
    (program.unwrap(), fragment_source)
}

fn make_basic_array_buffer(gpu: &Gpu) -> Agent<Buffer<f32>> {
    #[rustfmt::skip]
    let buffer = gpu.array_buffer(vec![
        0.,  0.,  0.,
        0., 0.8,  0.,
        0.8,  0., 0.,
    ]);
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
    buffer.unwrap()
}

pub fn draw_basic_elements(gpu: &Gpu) -> Result<(Agent<Elements>, Ace<String>), String> {
    let (program, vertex_source) = make_basic_program(&gpu);
    let buffer = make_basic_array_buffer(&gpu);
    let element_buffer = gpu.element_buffer(vec![0, 1, 2]).unwrap();
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.size(3);
    })?;
    let vao = gpu.vao(&vec![att]).unwrap();
    vao.write(|Pack { unit, back }| {
        unit.element_buffer(element_buffer.backed(back));
    })?;
    let elements = gpu.elements(&program, &buffer, &vao);
    elements.write(|pack| {
        pack.unit.count(3);
    })?;
    elements.act()?;
    Ok((elements, vertex_source))
}

////////////////////////////////////// Tests

pub fn make_vertex_shader() {
    let gpu = make_canvas();
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_fragment_shader() {
    let gpu = make_canvas();
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_program() {
    let gpu = make_canvas();
    make_basic_program(&gpu);
}

pub fn make_array_buffer() {
    let gpu = make_canvas();
    make_basic_array_buffer(&gpu);
}

pub fn make_element_buffer() {
    let gpu = make_canvas();
    #[rustfmt::skip]
    let buffer = gpu.element_buffer(vec![0,  1,  2]);
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
}

pub fn make_vertex_attribute() {
    let gpu = make_canvas();
    let buffer = make_basic_array_buffer(&gpu);
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.index(0).size(3).stride(0).offset(0);
    })
    .ok();
}

pub fn make_vertex_array_object() {
    let gpu = make_canvas();
    let buffer = make_basic_array_buffer(&gpu);
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.size(3);
    })
    .ok();
    let voa = gpu.vao(&vec![att]);
    if let Err(memo) = voa {
        panic!("gpu error: {memo}");
    }
}

pub fn draw_elements() -> react::Result {
    let gpu = make_canvas_on_body();
    draw_basic_elements(&gpu)?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source.
pub fn elements_react_to_shader_source() -> react::Result {
    let gpu = make_canvas_on_body();
    let (_elements, shader_source) = draw_basic_elements(&gpu)?;
    shader_source.write(|unit| *unit = shader::basic::FRAGMENT_GREEN.to_owned())?;
    Ok(())
}

/// Because elements has not been dropped yet, it should react to the change of shader source
/// and attempt to compile the shader. Error is expected.
pub fn shader_source_error() -> react::Result {
    let gpu = make_canvas();
    let (_elements, shader_source) = draw_basic_elements(&gpu)?;
    if let Err(_) = shader_source.write(|unit| *unit = "bad shader".to_owned()) {
        Ok(())
    } else {
        panic!("this shader write should have caused compile error");
    }
}


