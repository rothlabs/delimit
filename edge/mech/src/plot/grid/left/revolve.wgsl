struct Rig {
    // rename to dimension?
    order: u32,
    count: u32,
    offset: u32,
};

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> form: array<f32>;
@group(0) @binding(2) var<storage, read_write> weft: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    index: vec3<u32>
) {
    let order = rig.order;
    let weft_idx = rig.offset + index.x * order * order;

    let parameter = 0.;
    let cos_p = cos(parameter);
    let n_cos_p = 1 - cos_p;

    for (var k = 0u; k < order; k++) {
        for (var j = 0u; j < order; j++) {
            let w = weft_idx + j + k * order;
            if(j == k) {
                weft[w] = n_cos_p + cos_p;
            }
        }
    }
}