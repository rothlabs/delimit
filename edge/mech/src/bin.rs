use super::*;

#[derive(Debug)]
pub struct Bin {
    pub plot: PlotBin
}

impl Bin {
    pub fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            plot: PlotBin::new(gpu)?
        })
    }
}

#[derive(Debug)]
pub struct PlotBin {
    pub grid: GridPlotBin
}

impl PlotBin {
    fn new(gpu: &Gpu) -> graph::Result<Self> {
        Ok(Self {
            grid: GridPlotBin::new(gpu)?
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
            basis: GridPlotBasisBin::new(gpu)?
        })
    }
}

#[derive(Debug)]
pub struct GridPlotBasisBin {
    pub nurbs: Program,
    pub control: Program,
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
        let nurbs = Program { layout, pipe };
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
        let control = Program { layout, pipe };
        Ok(Self {
            nurbs,
            control
        })
    }
}

#[derive(Debug)]
pub struct Program {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}