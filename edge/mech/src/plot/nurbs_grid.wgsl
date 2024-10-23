struct Setup {
    order: u32,
    count: u32,
};

@group(0) @binding(0) var<uniform> setup: Setup;
@group(0) @binding(1) var<storage, read> plan: array<f32>;
@group(0) @binding(2) var<storage, read_write> basis: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    global: vec3<u32>
) {
    // prelude
    let plan_index = global.x / setup.count;
    let plot_index = global.x % setup.count;
    let order = setup.order;
    let degree = order - 1;
    let row_len = order * 3;
    let row = plan_index * row_len;

    // parameter and indices
    let parameter = f32(plot_index) / f32(setup.count - 1);
    let knot_index = row + degree;
    let weight_index = row + row_len - 1;
    let basis_index = global.x * order * 2 + degree;

    // basis reset [0., 0., 0., ..., 1.]
    basis[basis_index] = 1.;
    for (var i = 1u; i < order; i++) {
        basis[basis_index - i] = 0.;
    }

    // knot interpolation
    for (var deg = 1u; deg < order; deg++) {
        for (var i = 0u; i < deg + 1; i++) {
            let k0 = knot_index + i; 
            let k1 = k0 + 1;
            let n0 = basis_index + i - deg;
            let n1 = n0 + 1;
            var basis0 = 0.;
            var basis1 = 0.;
            if basis[n0] > 0. {
                let distance = plan[k0] - plan[k0 - deg];
                basis0 += basis[n0] * (parameter - plan[k0 - deg]) / distance; 
                basis1 += basis[n0] * f32(deg) / distance;
            }
            if basis[n1] > 0. && n1 <= basis_index {
                let distance = plan[k1] - plan[k1 - deg];
                basis0 += basis[n1] * (plan[k1] - parameter) / distance;
                basis1 -= basis[n1] * f32(deg) / distance;
            } 
            basis[n0        ] = basis0; 
            basis[n0 + order] = basis1;
        }
    }

    // weighted sum
    var sum0 = 0.;
    var sum1 = 0.;
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = basis_index - i;
        let b1 = b0 + order;
        sum0 += basis[b0] * plan[wi];
        sum1 += basis[b1] * plan[wi];
    }

    // rational
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = basis_index - i;
        let b1 = b0 + order;
        basis[b1] = (basis[b1] * sum0 - basis[b0] * sum1) * plan[wi] / sum0 / sum0;
        basis[b0] *= plan[wi] / sum0;
    }
}