struct Rig {
    order: u32,
    count: u32,
};

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> span: array<f32>;
@group(0) @binding(2) var<storage, read_write> basis: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    global: vec3<u32>
) {
    // prelude
    let order = rig.order;
    let span_index = global.x / rig.count;
    let plot_index = global.x % rig.count;
    let degree = order - 1;
    let row_len = order * 3;
    let row = span_index * row_len;

    // parameter and indices
    let parameter = f32(plot_index) / f32(rig.count - 1);
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
            // var basis2 = 0.; (acceleration)
            if basis[n0] > 0. {
                let distance = span[k0] - span[k0 - deg];
                basis0 += basis[n0] * (parameter - span[k0 - deg]) / distance; 
                basis1 += basis[n0] * f32(deg) / distance;
                // basis2 ...
            }
            if basis[n1] > 0. && n1 <= basis_index {
                let distance = span[k1] - span[k1 - deg];
                basis0 += basis[n1] * (span[k1] - parameter) / distance;
                basis1 -= basis[n1] * f32(deg) / distance;
                // basis2 ...
            } 
            basis[n0        ] = basis0; 
            basis[n0 + order] = basis1;
            // basis[n0 + order * 2] = basis2
        }
    }

    // weighted sum
    var sum0 = 0.;
    var sum1 = 0.;
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = basis_index - i;
        let b1 = b0 + order;
        sum0 += basis[b0] * span[wi];
        sum1 += basis[b1] * span[wi];
        // sum2 ...
    }

    // rational
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = basis_index - i;
        let b1 = b0 + order;
        // basis[b2] ...
        basis[b1] = (basis[b1] * sum0 - basis[b0] * sum1) * span[wi] / sum0 / sum0;
        basis[b0] *= span[wi] / sum0;
    }
}