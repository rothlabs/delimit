use client::*;
use gpu::*;
use graph::*;

// pub fn make_vertex_shader() {
//     let canvas = Canvas::new();
//     match canvas.gl().shader().vertex(shader::basic::VERTEX) {
//         Result::Err(memo) => panic!("gpu error: {memo}"),
//         _ => (),
//     };
// }

// pub fn make_vertex_shader() {
//     let canvas = Canvas::new();
//     match canvas.gl().shader().vertex(shader::basic::VERTEX) {
//         Result::Err(memo) => panic!("gpu error: {memo}"),
//         _ => (),
//     };
// }

// pub fn make_fragment_shader() {
//     let canvas = Canvas::new();
//     match canvas.gl().shader().fragment(shader::basic::FRAGMENT) {
//         Result::Err(memo) => panic!("gpu error: {memo}"),
//         _ => (),
//     };
// }

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
