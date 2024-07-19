use client::*;
use graph::*;
use gpu::*;

pub fn create_vertex_shader() {
    let gpu = GPU::new();
    match gpu.shader().vertex(shader::basic::VERTEX) {
        Result::Ok(_) => (),
        Result::Err(memo) => panic!("gpu error: {memo}"), 
    };
}

// #[test]
// fn make_basic_program() {
//     let gpu = GPU::new();
//     let program = gpu.program(shader::basic::VERTEX, shader::basic::FRAGMENT);
// }

pub fn rect() {
    let facets = draw::Facets::new();
    facets.link.grant();
}