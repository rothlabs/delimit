struct Rig {
    stride: u32,
}

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> plot: array<f32>;

struct Mesh {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    mesh: Mesh,
    @builtin(instance_index) index: u32, 
) -> Vertex {
    var out: Vertex;
    let position = mesh.position + plot[index * rig.stride];
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.color = mesh.color;
    return out;
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
    return vertex.color;
}