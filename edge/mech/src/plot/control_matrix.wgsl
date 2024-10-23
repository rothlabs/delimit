struct Setup {
    order: u32,
    // number of basis in a block
    count: u32,
    // number of plots in a control
    stride: u32,
    dimension: u32,
};

@group(0) @binding(0) var<uniform> setup: Setup;
@group(0) @binding(1) var<storage, read> index: array<u32>;
@group(0) @binding(2) var<storage, read> basis: array<f32>;
@group(0) @binding(3) var<storage, read> control: array<f32>;
@group(0) @binding(4) var<storage, read_write> plot: array<f32>;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) 
    global: vec3<u32>
) {
    // prelude
    let order = setup.order;
    let count = setup.count;
    let stride = setup.stride;
    let dimension = setup.dimension;

    // index
    let strip_index = global.x / count;
    let block_index = (strip_index / stride) * order;
    let local_index = strip_index % stride;
    let basis_index = block_index * count + (global.x % count) * order * 2;
    let plot_index = global.x * dimension * 2;
    
    // matrix multiplication
    for (var d = 0u; d < dimension; d++) {
        plot[plot_index + d] = 0.;
        for (var o = 0u; o < order; o++) {
            let control_index = index[block_index + o] * stride + local_index;
            let basis0 = basis[basis_index + o * stride];
            let control0 = control[control_index + d];
            plot[plot_index + d] += basis0 * control0;
        }
    }
}