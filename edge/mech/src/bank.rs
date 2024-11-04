use super::*;

mod mesh;

#[derive(Debug)]
pub struct Bank {
    pub plot: PlotBin,
}

impl Bank {
    pub fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            plot: PlotBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct PlotBin {
    pub grid: GridPlotBin,
    pub draw: DrawPlotBin,
}

impl PlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            grid: GridPlotBin::new(gpu)?,
            draw: DrawPlotBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct GridPlotBin {
    pub right: GridPlotRightBin,
}

impl GridPlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            right: GridPlotRightBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct GridPlotRightBin {
    pub nurbs: ComputeProgram,
    pub weave: ComputeProgram,
}

impl GridPlotRightBin {
    pub fn new(gpu: &Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/right/nurbs.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let span = gpu.bind_storage(true).entry(1)?.compute()?;
        let basis = gpu.bind_storage(false).entry(2)?.compute()?;
        let layout = gpu.bind_layout(&[rig, span, basis]).make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let nurbs = ComputeProgram { layout, pipe };
        let shader = gpu.shader(include_wgsl!("plot/grid/right/weave.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let warp = gpu.bind_storage(true).entry(1)?.compute()?;
        let weft = gpu.bind_storage(true).entry(2)?.compute()?;
        let flow = gpu.bind_storage(true).entry(3)?.compute()?;
        let plot = gpu.bind_storage(false).entry(4)?.compute()?;
        let layout = gpu
            .bind_layout(&[rig, warp, weft, flow, plot])
            .make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let weave = ComputeProgram { layout, pipe };
        Ok(Self { nurbs, weave })
    }
}

#[derive(Debug)]
pub struct DrawPlotBin {
    pub points: RenderMeshProgram,
}

impl DrawPlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/draw/points.wgsl"));
        let targets = gpu.display.targets();
        let rig = gpu.bind_uniform().entry(0)?.vertex()?;
        let plot = gpu.bind_storage(true).entry(1)?.vertex()?;
        let layout = gpu.bind_layout(&[rig, plot]).make()?;
        // TODO: put pipe_layout method on bind_layout
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let attribs = vertex_attr_array![0 => Float32x2];
        let buffers = vec![gpu.vertex_layout(8).attributes(&attribs).make()?];
        let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
        let fragment = shader.fragment("fs_main").targets(targets).make()?;
        let multi = gpu.multisample(4).make()?;
        let pipe = gpu
            .render_pipe(vertex)
            .fragment(fragment)
            .layout(&pipe_layout)
            .multisample(multi)
            .make()?;
        let count: u32 = 8;
        let points = mesh::Circle {
            count: count.into(),
            radius: 5.0.into(),
            display: gpu.display.config.clone(),
        }
        .gate()?;
        let buffer = gpu.buffer(count as u64 * 24).vertex()?;
        let mesh = Hedge {
            root: gpu.writer(buffer.clone()).data(points).hub()?,
            buffer: buffer.into(),
        };
        let points = RenderMeshProgram {
            layout,
            pipe,
            mesh,
            vertex_count: count * 3,
        };
        Ok(Self { points })
    }
}

#[derive(Debug)]
pub struct ComputeProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}

#[derive(Debug)]
pub struct RenderMeshProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<RenderPipeline>,
    pub mesh: Hedge,
    pub vertex_count: u32,
}
