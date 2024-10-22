struct Config {
    order: u32,
    count: u32,
};

@group(0) @binding(0) var<storage, read> nurbs: array<f32>;
@group(0) @binding(1) var<storage, read_write> basis0: array<f32>;
@group(0) @binding(2) var<uniform> config: Config;

@compute
@workgroup_size(1)
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
    let basis_index = (curve_index + plot_index) * config.order + degree;
    for (var i = 1u; i < config.order; i++) {
        basis0[basis_index - i] = 0.;
    }
    basis0[basis_index] = 1.;
    for (var deg = 1u; deg < config.order; deg++) {
        for (var i = 0u; i < deg + 1; i++) {
            let k0 = knot_index + i; 
            let k1 = k0 + 1;
            let b0 = basis_index + i - deg;
            let b1 = b0 + 1;
            var position = 0.;
            //var velocity = 0.;
            if basis0[b0] > 0. {
                let distance = nurbs[k0] - nurbs[k0 - deg];
                position += basis0[b0] * (param - nurbs[k0 - deg]) / distance; 
                //velocity += basis0[b0] * f32(deg) / distance;
            }
            if basis0[b1] > 0. && b1 <= basis_index {
                let distance = nurbs[k1] - nurbs[k1 - deg];
                position += basis0[b1] * (nurbs[k1] - param) / distance;
                //velocity -= basis0[b1] * f32(deg) / distance;
            } 
            basis0[b0] = position; 
        }
    }
}