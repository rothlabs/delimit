// use client::*;
use gpu::*;
use graph::*;
// use text::*;

pub fn make_vertex_shader() {
    let canvas = Canvas::link();
    let gpu = canvas.read(|unit| unit.gpu());
    if let Err(memo) = gpu.vertex_shader(shader::basic::VERTEX) {
        panic!("gpu error: {memo}");
    }
}

pub fn make_fragment_shader() {
    let canvas = Canvas::link();
    let gpu = canvas.read(|unit| unit.gpu());
    if let Err(memo) = gpu.fragment_shader(shader::basic::FRAGMENT) { 
        panic!("gpu error: {memo}");
    }
}

pub fn make_program() {
    let canvas = Canvas::link();
    let gpu = canvas.read(|unit| unit.gpu());
    let vertex = gpu.vertex_shader(shader::basic::VERTEX).unwrap();
    let fragment = gpu.fragment_shader(shader::basic::FRAGMENT).unwrap();
    if let Err(memo) = gpu.program(&vertex, &fragment) {
        panic!("gpu error: {memo}");
    }
}


// pub fn make_program() {
//     let canvas = Canvas::new();
//     let vertex_shader = canvas.gl().shader().vertex(shader::basic::VERTEX).unwrap();
//     let fragment_shader = canvas
//         .gl()
//         .shader()
//         .fragment(shader::basic::FRAGMENT)
//         .unwrap();
//     match canvas.gl().program(&vertex_shader, &fragment_shader) {
//         Result::Err(memo) => panic!("gpu error: {memo}"),
//         _ => (),
//     }
// }

// pub fn rect() {
//     let facets = draw::Facets::new();
//     facets.link.grant();
// }
