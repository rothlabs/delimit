use super::*;

#[derive(Debug)]
pub struct Bank {
    pub plot: PlotBin,
}

impl Bank {
    pub fn new(gpu: &Gpu) -> Result<Self> {
        Ok(Self {
            plot: PlotBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct PlotBin {
    pub grid: GridPlotBin,
}

impl PlotBin {
    fn new(gpu: &Gpu) -> Result<Self> {
        Ok(Self {
            grid: GridPlotBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct GridPlotBin {
    pub spin: SpinGridPlotBin,
    pub weave: WeaveGridPlotBin,
}

impl GridPlotBin {
    fn new(gpu: &Gpu) -> Result<Self> {
        Ok(Self {
            spin: SpinGridPlotBin::new(gpu)?,
            weave: WeaveGridPlotBin::new(gpu)?,
        })
    }
}

#[derive(Debug)]
pub struct SpinGridPlotBin {
    // travel
    pub extrude: ComputeProgram,

    // orient
    pub revolve2: ComputeProgram,

    // spline
    pub basis: ComputeProgram,
    pub nurbs: ComputeProgram,
}

impl SpinGridPlotBin {
    pub fn new(gpu: &Gpu) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/spin.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let form = gpu.bind_storage(true).entry(1)?.compute()?;
        let weft = gpu.bind_storage(false).entry(2)?.compute()?;
        let layout = gpu.bind_layout(&[rig, form, weft]).make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let extrude = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("extrude").layout(&pipe_layout).make()?,
        };
        let revolve2 = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("revolve2").layout(&pipe_layout).make()?,
        };
        let basis = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("basis").layout(&pipe_layout).make()?,
        };
        let pipe = shader.compute("nurbs").layout(&pipe_layout).make()?;
        let nurbs = ComputeProgram { layout, pipe };
        Ok(Self {
            extrude,
            revolve2,
            basis,
            nurbs,
        })
    }
}

#[derive(Debug)]
pub struct WeaveGridPlotBin {
    pub travel: ComputeProgram,
    pub orient: ComputeProgram,
    pub spline: ComputeProgram,
}

impl WeaveGridPlotBin {
    pub fn new(gpu: &Gpu) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/weave.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let warp = gpu.bind_storage(true).entry(1)?.compute()?;
        let weft = gpu.bind_storage(true).entry(2)?.compute()?;
        let flow = gpu.bind_storage(true).entry(3)?.compute()?;
        let plot = gpu.bind_storage(false).entry(4)?.compute()?;
        let layout = gpu.bind_layout(&[rig, warp, weft, flow, plot]).make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let travel = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("travel").layout(&pipe_layout).make()?,
        };
        let orient = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("orient").layout(&pipe_layout).make()?,
        };
        let spline = ComputeProgram {
            layout: layout.clone(),
            pipe: shader.compute("spline").layout(&pipe_layout).make()?,
        };
        Ok(Self {
            travel,
            orient,
            spline,
        })
    }
}

#[derive(Debug)]
pub struct ComputeProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}
