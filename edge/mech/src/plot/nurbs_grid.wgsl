struct Config {
    order: u32,
    count: u32,
};

@group(0) @binding(0) var<storage, read> nurbs: array<f32>;
@group(0) @binding(1) var<storage, read_write> basis0: array<f32>;
@group(0) @binding(2) var<uniform> config: Config;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    global: vec3<u32>
) {
    let curve_index = global.x / config.count;
    let plot_index = global.x % config.count;
    let param = f32(plot_index) / f32(config.count - 1);
    let degree = config.order - 1;
    let row_len = config.order * 3;
    let knot_index = curve_index * row_len + degree;
    let basis_index = curve_index * config.order + degree;
    for (var deg = 1u; deg < config.order; deg++) {
        for (var i = 0u; i < deg + 1; i++) {
            let k0 = knot_index + i; 
            let k1 = k0 + 1;
            let b0 = basis_index + i - deg;
            let b1 = b0 + 1;
            var position = 0.;
            if basis0[b0] > 0. {
                let distance = nurbs[k0] - nurbs[k0 - deg];
                position += basis0[b0] * (param - nurbs[k0 - deg]) / distance; 
            }
            // make sure the basis0 buffer starts like this: [0,0,1,  0,0,1,  0,0,1]
            if b1 <= basis_index && basis0[b1] > 0. { 
                let distance = nurbs[k1] - nurbs[k1 - deg];
                position += basis0[b1] * (nurbs[k1] - param) / distance;
            } 
            basis0[b0] = position; 
        }
    }
}