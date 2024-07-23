use gpu::*;
use graph::*;
use mecha::*;

fn make_gpu() -> Gpu {
    let canvas = Canvas::link();
    canvas.read(|unit| unit.gpu())
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
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT).unwrap();
    if let Err(memo) = gpu.program(&vertex, &fragment) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_array_buffer() {
    let gpu = make_gpu();
    #[rustfmt::skip]
    let buffer = gpu.array_buffer(Array1D::new([9], vec![
        0.,  0.,  0.,
        10., 0.,  0., 
        0.,  10., 0.,
    ]));
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
}

pub fn make_element_buffer() {
    let gpu = make_gpu();
    #[rustfmt::skip]
    let buffer = gpu.element_buffer(Array1D::new([3], vec![
        0,  1,  3, 
    ]));
    if let Err(memo) = buffer {
        panic!("gpu error: {memo}");
    }
}

pub fn make_vertex_attribute() {
    let gpu = make_gpu();
    let att = gpu.vertex_attribute();
    att.write(|pack| {
        pack.unit.index(0).size(3).stride(0).offset(0);
    });
}

pub fn make_vertex_array_object() {
    let gpu = make_gpu();
    let att = gpu.vertex_attribute();
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
    let buffer = gpu.array_buffer(Array1D::new([9], vec![
        0.,  0.,  0.,
        10., 0.,  0., 
        0.,  10., 0.,
    ])).unwrap();
    let element_buffer = gpu.element_buffer(Array1D::new([3], vec![
        0,  1,  3, 
    ])).unwrap();
    let att = gpu.vertex_attribute();
    att.write(|pack| {
        pack.unit.size(3);
    });
    let vao = gpu.vao(&vec![att]).unwrap();
    vao.write(|Pack { unit, back }|{
        unit.element_buffer(element_buffer.backed(back));
    });
    let elements = gpu.elements(&buffer, &vao);
    elements.act();
}