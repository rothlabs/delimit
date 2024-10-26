struct Model {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Instance {
    @location(2) position: vec2<f32>,
}

struct VertexOutput {
    @location(0) color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
    model: Model,
    instance: Instance,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(model.position,    0.0, 1.0)
                 + vec4<f32>(instance.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vertex.color;
}