@group(0) 
@binding(0) 
var<uniform> count: u32;

@group(0) 
@binding(1)
var<storage, read_write> basis: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    index: vec3<u32>
) {
    basis[index.x] = f32(index.x) / f32(count - 1);
}