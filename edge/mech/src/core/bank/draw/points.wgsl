struct Rig {
    plot_stride: u32,
    vertex_offset: u32,
    plot_offset: u32,
    plot_length: u32,
}

struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@group(0) @binding(0) var<storage, read> data: array<f32>;
@group(1) @binding(0) var<storage, read> rig: Rig;

@vertex
fn vs_main(
    // mesh: Mesh,
    @builtin(vertex_index) vertex_index: u32, 
    @builtin(instance_index) instance_index: u32, 
) -> Vertex {
    let vertex_idx = rig.vertex_offset + vertex_index * 2;
    let vertex = vec2<f32>(data[vertex_idx], data[vertex_idx + 1]);
    let plot_idx = rig.plot_offset + instance_index * rig.plot_stride;
    let plot = vec2<f32>(data[plot_idx], data[plot_idx + 1]);
    let ratio = f32(instance_index) / f32(rig.plot_length - 1);
    var out: Vertex;
    out.position = vec4<f32>(plot + vertex, 0.0, 1.0);
    out.color = vec4<f32>(ratio, 0., 1. - ratio, 1.0);
    return out;
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
    return vertex.color;
}

// struct Mesh {
//     @location(0) position: vec2<f32>,
// }

// let vertex_mod = index % rig.vertex_length;
    // let vertex_div = index / rig.vertex_length;

// struct Rig {
//     // plot plot_stride
//     plot_stride: u32,
//     vertex_offset: u32,
//     // vertex_length: u32,
//     plot_offset: u32,
//     plot_length: u32,
// }

// @group(0) @binding(0) var<storage, read> data: array<f32>;
// @group(1) @binding(0) var<storage, read> rig: Rig;

// // struct Mesh {
// //     @location(0) position: vec2<f32>,
// // }

// struct Vertex {
//     @builtin(position) position: vec4<f32>,
//     @location(0) color: vec4<f32>,
// };

// @vertex
// fn vs_main(
//     // mesh: Mesh,
//     @builtin(instance_index) index: u32, 
// ) -> Vertex {
//     var out: Vertex;
//     let vertex_mod = index % rig.vertex_length;
//     let vertex = data[rig.vertex_offset + vertex_mod * 2];
//     let vertex_div = index / rig.vertex_length;
//     let plot_idx = rig.plot_offset + vertex_div * rig.plot_stride;
//     let position = vertex + vec2<f32>(data[plot_idx], data[plot_idx + 1]);
//     let ratio = f32(index) / f32(rig.plot_length - 1);
//     out.position = vec4<f32>(position, 0.0, 1.0);
//     out.color = vec4<f32>(ratio, 0., 1. - ratio, 1.0);
//     return out;
// }

// @fragment
// fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
//     return vertex.color;
// }