struct Rig {
    order: u32,
    // number of basis in a block
    count: u32,
    // number of plots in a control
    stride: u32,
    dimension: u32,
};

@group(0) @binding(0) var<uniform> rig: Rig;
@group(0) @binding(1) var<storage, read> basis: array<f32>;
@group(0) @binding(2) var<storage, read> index: array<u32>;
@group(0) @binding(3) var<storage, read> control: array<f32>;
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
    let stride = rig.stride;
    let dimension = rig.dimension;

    // index
    let strip_index = global.x / count;
    let block_index = strip_index / stride;
    let local_index = strip_index % stride;
    let basis_index = (block_index * count + global.x % count) * order * 2;
    // TODO: loop plot slots through rank, global.x * dimension + dimension * rank
    let plot_index0 = global.x * dimension * 2;
    let plot_index1 = plot_index0 + dimension;
    let idx = block_index * order;
    
    // matrix multiplication
    for (var d = 0u; d < dimension; d++) {
        plot[plot_index0 + d] = 0.;
        plot[plot_index1 + d] = 0.;
        for (var o = 0u; o < order; o++) {
            let control_index = (index[idx + o] * stride + local_index) * dimension;
            let basis0 = basis[basis_index + o];
            let basis1 = basis[basis_index + order + o];
            let control0 = control[control_index + d];
            // TODO: loop plot slots through rank
            plot[plot_index0 + d] += basis0 * control0;
            plot[plot_index1 + d] += basis1 * control0;
        }
    }
}