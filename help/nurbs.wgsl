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
    index: vec3<u32>
) {
    // prelude
    let order = rig.order;
    let count = rig.count;

    // index and parameter
    let count_idx = index.x / count;
    let count_mod = index.x % count;
    let degree = order - 1;
    let row_len = order * 3;
    let row_pos = count_idx * row_len;
    let knot_idx = row_pos + degree;
    let weight_idx = row_pos + row_len - 1;
    let weft_idx = index.x * order * 2 + degree + rig.offset;
    let start = form[knot_idx];
    let end = form[knot_idx + 1];
    let parameter = start + (end - start) * f32(count_mod) / f32(count - 1);

    // weft reset [0., 0., 0., ..., 1.]
    weft[weft_idx] = 1.;
    for (var i = 1u; i < order; i++) {
        weft[weft_idx - i] = 0.;
    }

    // knot interpolation
    for (var deg = 1u; deg < order; deg++) {
        for (var i = 0u; i < deg + 1; i++) {
            let k0 = knot_idx + i; 
            let k1 = k0 + 1;
            let n0 = weft_idx + i - deg;
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
            if weft[n1] > 0. && n1 <= weft_idx {
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
        let wi = weight_idx - i;
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        sum0 += weft[b0] * form[wi];
        sum1 += weft[b1] * form[wi];
        // sum2 ...
    }

    // rational
    for (var i = 0u; i < order; i++) {
        let wi = weight_idx - i;
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        // weft[b2] = ... / sum0 / sum0 / sum0;
        weft[b1] = (weft[b1] * sum0 - weft[b0] * sum1) * form[wi] / sum0 / sum0;
        weft[b0] *= form[wi] / sum0;
    }
}