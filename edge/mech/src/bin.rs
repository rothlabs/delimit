use super::*;

#[derive(Debug)]
pub struct Bin {
    pub plot: PlotBin,
}

impl Bin {
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
    pub basis: GridPlotBasisBin,
}

impl GridPlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            basis: GridPlotBasisBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct GridPlotBasisBin {
    pub nurbs: ComputeProgram,
    pub control: ComputeProgram,
}

impl GridPlotBasisBin {
    pub fn new(gpu: &Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/basis/nurbs.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let span = gpu.bind_storage(true).entry(1)?.compute()?;
        let basis = gpu.bind_storage(false).entry(2)?.compute()?;
        let layout = gpu.bind_layout(&[rig, span, basis]).make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let nurbs = ComputeProgram { layout, pipe };
        let shader = gpu.shader(include_wgsl!("plot/grid/basis/control.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let basis = gpu.bind_storage(true).entry(1)?.compute()?;
        let index = gpu.bind_storage(true).entry(2)?.compute()?;
        let control = gpu.bind_storage(true).entry(3)?.compute()?;
        let plot = gpu.bind_storage(false).entry(4)?.compute()?;
        let layout = gpu
            .bind_layout(&[rig, basis, index, control, plot])
            .make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let control = ComputeProgram { layout, pipe };
        Ok(Self { nurbs, control })
    }
}

#[derive(Debug)]
pub struct DrawPlotBin {
    pub points: RenderProgram,
}

impl DrawPlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/draw/points.wgsl"));
        let targets = gpu.surface.targets();
        let rig = gpu.bind_uniform().entry(0)?.vertex()?;
        let plot = gpu.bind_storage(true).entry(1)?.vertex()?;
        let layout = gpu.bind_layout(&[rig, plot]).make()?;
        // TODO: put pipe_layout method on bind_layout
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let attribs = vertex_attr_array![0 => Float32x2];
        let buffers = vec![gpu.vertex_layout(8).attributes(&attribs).make()?];
        let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
        let fragment = shader.fragment("fs_main").targets(targets).make()?;
        let pipe = gpu
            .render_pipe(vertex)
            .fragment(fragment)
            .layout(&pipe_layout)
            .make()?;
        let points = RenderProgram { layout, pipe };
        Ok(Self { points })
    }
}

#[derive(Debug)]
pub struct ComputeProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}

#[derive(Debug)]
pub struct RenderProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<RenderPipeline>,
}
