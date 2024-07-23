use gpu::*;
use graph::*;
use mecha::*;
// use wasm_bindgen_test::console_log;

fn make_gpu() -> Gpu {
    let canvas = Canvas::link();
    canvas.read(|unit| unit.gpu())
}

fn make_basic_program(gpu: &Gpu) -> (Agent<Program>, Ace<String>) {
    let vertex_shader = shader::basic::VERTEX.ace();
    let vertex = gpu.vertex_shader(&vertex_shader).unwrap();
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT).unwrap();
    let program = gpu.program(&vertex, &fragment);
    if let Err(memo) = program {
        panic!("gpu error: {memo}");
    }
    (program.unwrap(), vertex_shader)
}

fn make_basic_array_buffer(gpu: &Gpu) -> Agent<Buffer<f32>> {
    #[rustfmt::skip]
    let buffer = gpu.array_buffer(vec![
        0.,  0.,  0.,
        10., 0.,  0., 
        0.,  10., 0.,
    ].array());
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
    buffer.unwrap()
}

pub fn make_vertex_shader() {
    let gpu = make_gpu();
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_fragment_shader() {
    let gpu = make_gpu();
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_program() {
    let gpu = make_gpu();
    make_basic_program(&gpu);
}

pub fn make_array_buffer() {
    let gpu = make_gpu();
    make_basic_array_buffer(&gpu);
}

pub fn make_element_buffer() {
    let gpu = make_gpu();
    #[rustfmt::skip]
    let buffer = gpu.element_buffer(vec![
        0,  1,  3, 
    ].array());
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
}

pub fn make_vertex_attribute() {
    let gpu = make_gpu();
    let buffer = make_basic_array_buffer(&gpu);
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.index(0).size(3).stride(0).offset(0);
    });
}

pub fn make_vertex_array_object() {
    let gpu = make_gpu();
    let buffer = make_basic_array_buffer(&gpu);
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.size(3);
    });
    let voa = gpu.vao(&vec![att]);
    if let Err(memo) = voa {
        panic!("gpu error: {memo}");
    }
}

pub fn draw_elements() {
    let gpu = make_gpu();
    let (program, vertex_shader) = make_basic_program(&gpu);
    let buffer = make_basic_array_buffer(&gpu);
    let element_buffer = gpu
        .element_buffer(vec![0, 1, 3].array())
        .unwrap();
    let att = gpu.vertex_attribute(&buffer);
    att.write(|pack| {
        pack.unit.size(3);
    });
    let vao = gpu.vao(&vec![att]).unwrap();
    vao.write(|Pack { unit, back }| {
        unit.element_buffer(element_buffer.backed(back));
    });
    let elements = gpu.elements(&program, &buffer, &vao);
    elements.write(|pack| {
        pack.unit.count(3);
    });
    let draw_result = elements.act();
    if let Err(memo) = draw_result {
        panic!("gpu error: {memo}");
    }
    vertex_shader.write(|unit| *unit = "wow not a shader".to_owned());
}

// console_log!("draw elements");
// vertex_shader.write(|unit| *unit = "wow not a shader".to_owned());
// let draw_result = elements.act();
// if let Err(memo) = draw_result {
//     panic!("gpu error: {memo}");
// }
// console_log!("draw elements again?");
