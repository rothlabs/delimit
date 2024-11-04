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
    global: vec3<u32>
) {
    // prelude
    let order = rig.order;
    let count = rig.count;
    let area = rig.area;
    let dimension = rig.dimension;

    // index and modulo
    let count_idx = global.x / count;
    let count_mod = global.x % count;
    let area_idx = count_idx / area;
    let area_mod = count_idx % area;
    // TODO: loop plot slots through rank, global.x * dimension + dimension * rank
    let plot_idx0 = global.x * dimension * 2 + rig.offset;
    let plot_idx1 = plot_idx0 + dimension;
    let flow_idx = area_idx * (order + 1);
    let weft_idx = (flow[flow_idx] * count + count_mod) * order * 2;

    // matrix-vector multiplication
    for (var d = 0u; d < dimension; d++) {
        plot[plot_idx0 + d] = 0.;
        plot[plot_idx1 + d] = 0.;
        for (var o = 0u; o < order; o++) {
            let warp_index = (flow[flow_idx + o + 1] * area + area_mod) * dimension;
            let warp0 = warp[warp_index + d];
            let weft0 = weft[weft_idx + o];
            let weft1 = weft[weft_idx + order + o];
            // TODO: loop plot slots through rank
            plot[plot_idx0 + d] += warp0 * weft0;
            plot[plot_idx1 + d] += warp0 * weft1;
        }
    }
}