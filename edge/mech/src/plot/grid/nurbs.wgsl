struct Rig {
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
    global: vec3<u32>
) {
    // prelude
    let order = rig.order;
    let form_index = global.x / rig.count;
    let plot_index = global.x % rig.count;
    let degree = order - 1;
    let row_len = order * 3;
    let row = form_index * row_len;

    // parameter and indices
    let parameter = f32(plot_index) / f32(rig.count - 1);
    let knot_index = row + degree;
    let weight_index = row + row_len - 1;
    let weft_index = global.x * order * 2 + degree + rig.offset;

    // weft reset [0., 0., 0., ..., 1.]
    weft[weft_index] = 1.;
    for (var i = 1u; i < order; i++) {
        weft[weft_index - i] = 0.;
    }

    // knot interpolation
    for (var deg = 1u; deg < order; deg++) {
        for (var i = 0u; i < deg + 1; i++) {
            let k0 = knot_index + i; 
            let k1 = k0 + 1;
            let n0 = weft_index + i - deg;
            let n1 = n0 + 1;
            var weft0 = 0.;
            var weft1 = 0.;
            // var weft2 = 0.; (acceleration)
            if weft[n0] > 0. {
                let distance = form[k0] - form[k0 - deg];
                weft0 += weft[n0] * (parameter - form[k0 - deg]) / distance; 
                weft1 += weft[n0] * f32(deg) / distance;
                // weft2 ...
            }
            if weft[n1] > 0. && n1 <= weft_index {
                let distance = form[k1] - form[k1 - deg];
                weft0 += weft[n1] * (form[k1] - parameter) / distance;
                weft1 -= weft[n1] * f32(deg) / distance;
                // weft2 ...
            } 
            weft[n0        ] = weft0; 
            weft[n0 + order] = weft1;
            // weft[n0 + order * 2] = weft2
        }
    }

    // weighted sum
    var sum0 = 0.;
    var sum1 = 0.;
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = weft_index - i;
        let b1 = b0 + order;
        sum0 += weft[b0] * form[wi];
        sum1 += weft[b1] * form[wi];
        // sum2 ...
    }

    // rational
    for (var i = 0u; i < order; i++) {
        let wi = weight_index - i;
        let b0 = weft_index - i;
        let b1 = b0 + order;
        // weft[b2] = ... / sum0 / sum0 / sum0;
        weft[b1] = (weft[b1] * sum0 - weft[b0] * sum1) * form[wi] / sum0 / sum0;
        weft[b0] *= form[wi] / sum0;
    }
}