@group(0) 
@binding(0) 
var<storage, read_write> basis: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    u_id: vec3<u32>
) {
    basis[u_id.x] = f32(u_id.x) / 127.0;
}