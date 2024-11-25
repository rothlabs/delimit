struct Rig {
    order: u32,
    count: u32,
    offset: u32,
    length: u32,
};

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> form: array<f32>;
@group(0) @binding(2) var<storage, read_write> weft: array<f32>;

// Travel ----------------------------------------------------

@compute @workgroup_size(64)
fn extrude(@builtin(global_invocation_id) index: vec3<u32>) {
    let order = rig.order;
    let count_idx = index.x / rig.count;
    let count_mod = index.x % rig.count;
    let form_idx = count_idx * order;
    let weft_idx = rig.offset + index.x * order * 2;
    let parameter = f32(count_mod) / f32(rig.count - 1);
    for (var o = 0u; o < order; o++) {
        let component = form[form_idx + o];
        // position
        weft[weft_idx + o] = component * parameter;
        // velocity
        weft[weft_idx + o + order] = component;
    }
}

// Orient ----------------------------------------------------

@compute @workgroup_size(64)
fn revolve2(@builtin(global_invocation_id) index: vec3<u32>) {
    // let order = rig.order;
    let count_idx = index.x / rig.count;
    let count_mod = index.x % rig.count;
    let weft_idx = rig.offset + index.x * 8;
    let angle = form[count_idx] * f32(count_mod) / f32(rig.count - 1);
    // position
    weft[weft_idx]     =  cos(angle);
    weft[weft_idx + 1] =  sin(angle);
    weft[weft_idx + 2] = -sin(angle);
    weft[weft_idx + 3] =  cos(angle);
    // velocity, TODO: fill out velocity calc!
    weft[weft_idx + 4] = 0.;
    weft[weft_idx + 5] = 0.;
    weft[weft_idx + 6] = 0.;
    weft[weft_idx + 7] = 0.;
}

// Spline -------------------------------------------------

@compute @workgroup_size(64)
fn basis(@builtin(global_invocation_id) index: vec3<u32>) {
    // prelude
    let order = rig.order;
    let weft_idx = rig.offset + index.x * order * 2 + order - 1;

    // knot interpolation
    interpolate(index, weft_idx, 2u);

    // sum
    var sum = vec2(0., 0.);
    for (var i = 0u; i < order; i++) {
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        sum += vec2(weft[b0], weft[b1]);
    }

    // proportion
    for (var i = 0u; i < order; i++) {
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        // weft[b2] = ... / sum.x / sum.x / sum.x;
        weft[b1] = (weft[b1] * sum.x - weft[b0] * sum.y) / sum.x / sum.x;
        weft[b0] /= sum.x;
    }
}

@compute @workgroup_size(64)
fn nurbs(@builtin(global_invocation_id) index: vec3<u32>) {
    if(index.x > rig.length - 1) {
        return;
    }

    // prelude
    let order = rig.order;
    let count_idx = index.x / rig.count;
    let row_len = order * 3;
    let weight_idx = count_idx * row_len + row_len - 1;
    let weft_idx = rig.offset + index.x * order * 2 + order - 1;

    // knot interpolation
    interpolate(index, weft_idx, 3u);

    // weighted sum
    var sum = vec2(0., 0.);
    for (var i = 0u; i < order; i++) {
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        let wi = weight_idx - i;
        sum += vec2(weft[b0], weft[b1]) * form[wi];
    }

    // rational proportion
    for (var i = 0u; i < order; i++) {
        let wi = weight_idx - i;
        let b0 = weft_idx - i;
        let b1 = b0 + order;
        // weft[b2] = ... / sum0 / sum0 / sum0;
        weft[b1] = (weft[b1] * sum.x - weft[b0] * sum.y) * form[wi] / sum.x / sum.x;
        weft[b0] *= form[wi] / sum.x;
    }
}

fn interpolate(index: vec3<u32>, weft_idx: u32, row_len: u32) {
    // prelude
    let order = rig.order;
    let count_idx = index.x / rig.count;
    let count_mod = index.x % rig.count;
    let row_pos = count_idx * order * row_len;
    let knot_idx = row_pos + order - 1;
    let start = form[knot_idx];
    let end = form[knot_idx + 1];
    let parameter = start + (end - start) * f32(count_mod) / f32(rig.count - 1);

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
            // position
            var weft0 = 0.;
            // velocity
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
}