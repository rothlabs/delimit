struct Rig {
    rank: u32,
    order: u32,
    offset: u32,
    // number of weft in a block
    count: u32,
    // number of plots in a warp
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

    // index
    let strip_index = global.x / count;
    let block_index = strip_index / area;
    let local_index = strip_index % area;
    let weft_index = (block_index * count + global.x % count) * order * 2;
    // TODO: loop plot slots through rank, global.x * dimension + dimension * rank
    let plot_index0 = global.x * dimension * 2;
    let plot_index1 = plot_index0 + dimension;
    let flow_index = block_index * order;
    
    // matrix-vector multiplication
    for (var d = 0u; d < dimension; d++) {
        plot[plot_index0 + d] = 0.;
        plot[plot_index1 + d] = 0.;
        for (var o = 0u; o < order; o++) {
            let warp_index = (flow[flow_index + o] * area + local_index) * dimension;
            let warp0 = warp[warp_index + d];
            let weft0 = weft[weft_index + o];
            let weft1 = weft[weft_index + order + o];
            // TODO: loop plot slots through rank
            plot[plot_index0 + d] += warp0 * weft0;
            plot[plot_index1 + d] += warp0 * weft1;
        }
    }
}