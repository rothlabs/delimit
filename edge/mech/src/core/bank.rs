use super::*;

// mod bind;
mod draw;

#[derive(Debug)]
pub struct Bank {
    pub plot: PlotBank,
    pub draw: DrawBank,
}

impl Bank {
    pub fn new(port: &Viewport) -> Result<Self> {
        Ok(Self {
            plot: PlotBank::new(&port.gpu)?,
            draw: DrawBank::new(port)?,
        })
    }
}

#[derive(Debug)]
pub struct PlotBank {
    pub grid: GridPlotBin,
}

impl PlotBank {
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
        let layout = gpu
            .pipe_layout(&[&gpu.store.storage.layout, &gpu.store.uniform.layout])
            .make()?;
        Ok(Self {
            spin: SpinGridPlotBin::new(gpu, &layout)?,
            weave: WeaveGridPlotBin::new(gpu, &layout)?,
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
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/spin.wgsl"));
        let extrude = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("extrude").layout(layout).make()?,
        };
        let revolve2 = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("revolve2").layout(layout).make()?,
        };
        let basis = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("basis").layout(layout).make()?,
        };
        let pipe = shader.compute("nurbs").layout(layout).make()?;
        let nurbs = ComputeProgram { pipe };
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
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/weave.wgsl"));
        // let rig = gpu.bind_uniform().entry(0)?.compute()?;
        // let warp = gpu.bind_storage(true).entry(1)?.compute()?;
        // let weft = gpu.bind_storage(true).entry(2)?.compute()?;
        // let flow = gpu.bind_storage(true).entry(3)?.compute()?;
        // let plot = gpu.bind_storage(false).entry(4)?.compute()?;
        // let layout = gpu.bind_layout(&[rig, warp, weft, flow, plot]).make()?;
        // let layout = gpu.pipe_layout(&[&gpu.store.storage.layout, &gpu.store.uniform.layout]).make()?;
        let travel = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("travel").layout(layout).make()?,
        };
        let orient = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("orient").layout(layout).make()?,
        };
        let spline = ComputeProgram {
            // layout: layout.clone(),
            pipe: shader.compute("spline").layout(layout).make()?,
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
    // pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}

///////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct DrawBank {
    pub chart: draw::ChartBank,
}

impl DrawBank {
    pub fn new(port: &Viewport) -> Result<Self> {
        let store = &port.gpu.store;
        let layout = port
            .gpu
            .pipe_layout(&[&store.storage.layout_vertex, &store.uniform.layout_vertex])
            .make()?;
        Ok(Self {
            chart: draw::ChartBank::new(port, &layout)?,
        })
    }
}

#[derive(Debug)]
pub struct RenderProgram {
    // pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<RenderPipeline>,
}

///////////////////////
