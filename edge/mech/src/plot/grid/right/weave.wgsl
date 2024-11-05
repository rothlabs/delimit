struct Rig {
    rank: u32,
    order: u32,
    offset: u32,
    count: u32,
    area: u32,
    dimension: u32,
};

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> warp: array<f32>;
@group(0) @binding(2) var<storage, read> weft: array<f32>;
@group(0) @binding(3) var<storage, read> flow: array<u32>;
@group(0) @binding(4) var<storage, read_write> plot: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    index: vec3<u32>
) {
    // prelude
    let rank = rig.rank;
    let order = rig.order;
    let count = rig.count;
    let area = rig.area;
    let dimension = rig.dimension;
    let order2 = order * 2;
    let dimension2 = dimension * 2;
    let warp_stride = dimension * (rank + 1);
    let plot_stride = dimension * (rank + 2);

    // index and modulo
    let count_idx = index.x / count;
    let count_mod = index.x % count;
    let area_idx = count_idx / area;
    let area_mod = count_idx % area;
    let plot_idx = index.x * plot_stride + rig.offset;
    let flow_idx = area_idx * (order + 1);
    let weft_idx = (flow[flow_idx] * count + count_mod) * order2;

    // reset plot
    for (var i = 0u; i < plot_stride; i++) {
        plot[plot_idx + i] = 0.;
    }

    // matrix-vector multiplication
    for (var d = 0u; d < dimension; d++) {
        for (var o = 0u; o < order; o++) {
            let warp_idx = (flow[flow_idx + o + 1] * area + area_mod) * warp_stride;
            let warp0 = warp[warp_idx + d];
            let weft0 = weft[weft_idx + o];

            // position
            plot[plot_idx + d] += warp0 * weft0;
            // velocity 
            let weft1 = weft[weft_idx + order + o];
            plot[plot_idx + dimension + d] += warp0 * weft1;
            // acceleration 
            // let weft2 = weft[weft_idx + order2 + o];
            // plot[plot_idx + dimension2 + d] += warp0 * weft2;

            // progenitor quantities 
            for (var r = 0u; r < rank; r++) {
                let warp0 = warp[warp_idx + dimension2 + dimension * r + d];
                plot[plot_idx + dimension2 + dimension * r + d] += warp0 * weft0;
            }
        }
    }
}