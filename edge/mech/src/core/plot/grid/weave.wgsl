// struct Rig {
//     rank: u32,
//     order: u32,
//     offset: u32,
//     length: u32,
//     count: u32,
//     area: u32,
//     dimension: u32,
// };
struct Rig {
    rank: u32,
    order: u32,
    count: u32,
    area: u32,
    dimension: u32,
    warp_offset: u32,
    weft_offset: u32,
    flow_offset: u32,
    plot_offset: u32,
    length: u32,
};

// @group(0) @binding(1) var<storage, read> warp: array<f32>;
// @group(0) @binding(2) var<storage, read> weft: array<f32>;
// @group(0) @binding(3) var<storage, read> flow: array<u32>;
@group(0) @binding(0) var<storage, read_write> data: array<f32>;
// @group(0) @binding(1) var<storage, read> data_u32: array<u32>;

@group(1) @binding(0) var<uniform> rig: Rig;

@compute @workgroup_size(64)
fn travel(@builtin(global_invocation_id) index: vec3<u32>) {
    if(index.x > rig.length - 1) {
        return;
    }
    
    // prelude
    let rank = rig.rank;
    let count = rig.count;
    let area = rig.area;
    let dimension = rig.dimension;
    let weft_stride = dimension * 2;
    let warp_stride = dimension * (rank + 1);
    let plot_stride = dimension * (rank + 2);

    // index and modulo
    let count_idx = index.x / count;
    let count_mod = index.x % count;
    let area_idx = count_idx / area;
    let area_mod = count_idx % area;
    let plot_idx = rig.plot_offset + index.x * plot_stride;
    let flow_idx = rig.flow_offset + area_idx * 2;
    let weft_idx = rig.weft_offset + (u32(data[flow_idx]) * count + count_mod) * weft_stride;
    let warp_idx = rig.warp_offset + (u32(data[flow_idx + 1]) * area + area_mod) * warp_stride;

    // translation
    for (var d = 0u; d < dimension; d++) {
        let warp0 = data[warp_idx + d];
        let weft0 = data[weft_idx + d];

        // position
        data[plot_idx + d] = warp0 + weft0;
        // velocity 
        let weft1 = data[weft_idx + dimension + d];
        data[plot_idx + dimension + d] = warp0 + weft1;
        // acceleration 
        // let weft2 = data[weft_idx + order2 + o];
        // data[plot_idx + weft_stride + d] += warp0 * weft2;

        // progenitor quantities 
        for (var r = 0u; r < rank; r++) {
            let warp0 = data[warp_idx + weft_stride + dimension * r + d];
            data[plot_idx + weft_stride + dimension * r + d] = warp0;
        }
    }
}

@compute @workgroup_size(64)
fn orient(@builtin(global_invocation_id) index: vec3<u32>) {
    if(index.x > rig.length - 1) {
        return;
    }

    // prelude
    let rank = rig.rank;
    // let order = rig.order;
    let count = rig.count;
    let area = rig.area;
    let dimension = rig.dimension;
    // let order2 = order * 2;
    let weft_block = dimension * dimension;
    let weft_stride = weft_block * 2;
    let warp_stride = dimension * (rank + 1);
    let plot_stride = dimension * (rank + 2);
    let plot_block = dimension * 2;

    // index and modulo
    let count_idx = index.x / count;
    let count_mod = index.x % count;
    let area_idx = count_idx / area;
    let area_mod = count_idx % area;
    let plot_idx = rig.plot_offset + index.x * plot_stride;
    let flow_idx = rig.flow_offset + area_idx * 2;
    let weft_idx = rig.weft_offset + (u32(data[flow_idx]) * count + count_mod) * weft_stride;
    let warp_idx = rig.warp_offset + (u32(data[flow_idx + 1]) * area + area_mod) * warp_stride;

    // reset plot
    for (var i = 0u; i < plot_stride; i++) {
        data[plot_idx + i] = 0.;
    }

    // matrix-vector multiplication
    for (var c = 0u; c < dimension; c++) {
        let warp0 = data[warp_idx + c];
        let col = dimension * c;
        for (var d = 0u; d < dimension; d++) {
            let weft0 = data[weft_idx + col + d];

            // position
            data[plot_idx + d] += warp0 * weft0;
            // velocity 
            let weft1 = data[weft_idx + weft_block + col + d];
            data[plot_idx + dimension + d] += warp0 * weft1;
            // acceleration 
            // let weft2 = data[weft_idx + order2 + d];
            // data[plot_idx + plot_block + d] += warp0 * weft2;

            // progenitor quantities 
            for (var r = 0u; r < rank; r++) {
                let warp0 = data[warp_idx + plot_block + dimension * r + d];
                data[plot_idx + plot_block + dimension * r + d] += warp0 * weft0;
            }
        }
    }
}

@compute @workgroup_size(64)
fn spline(@builtin(global_invocation_id) index: vec3<u32>) {
    if(index.x > rig.length - 1) {
        return;
    }

    // prelude
    let rank = rig.rank;
    let order = rig.order;
    let count = rig.count;
    let area = rig.area;
    let dimension = rig.dimension;
    let order2 = order * 2;
    let warp_stride = dimension * (rank + 1);
    let plot_stride = dimension * (rank + 2);
    let plot_block = dimension * 2;

    // index and modulo
    let count_idx = index.x / count;
    let count_mod = index.x % count;
    let area_idx = count_idx / area;
    let area_mod = count_idx % area;
    let plot_idx = rig.plot_offset + index.x * plot_stride;
    let flow_idx = rig.flow_offset + area_idx * (order + 1);
    let weft_idx = rig.weft_offset + (u32(data[flow_idx]) * count + count_mod) * order2;

    // reset plot
    for (var i = 0u; i < plot_stride; i++) {
        data[plot_idx + i] = 0.;
    }

    // matrix-vector multiplication
    for (var d = 0u; d < dimension; d++) {
        for (var o = 0u; o < order; o++) {
            let warp_idx = rig.warp_offset + (u32(data[flow_idx + o + 1]) * area + area_mod) * warp_stride;
            let warp0 = data[warp_idx + d];
            let weft0 = data[weft_idx + o];

            // position
            data[plot_idx + d] += warp0 * weft0;
            // velocity 
            let weft1 = data[weft_idx + order + o];
            data[plot_idx + dimension + d] += warp0 * weft1;
            // acceleration 
            // let weft2 = data[weft_idx + order2 + o];
            // data[plot_idx + plot_block + d] += warp0 * weft2;

            // progenitor quantities 
            for (var r = 0u; r < rank; r++) {
                let warp0 = data[warp_idx + plot_block + dimension * r + d];
                data[plot_idx + plot_block + dimension * r + d] += warp0 * weft0;
            }
        }
    }
}