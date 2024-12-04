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
            .pipe_layout(&[&gpu.store.topic.layout, &gpu.store.rig.layout])
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
    pub extrude: Grc<ComputePipeline>,

    // orient
    pub revolve2: Grc<ComputePipeline>,

    // spline
    pub basis: Grc<ComputePipeline>,
    pub nurbs: Grc<ComputePipeline>,
}

impl SpinGridPlotBin {
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/spin.wgsl"));
        let extrude = shader.compute("extrude").layout(layout).make()?;
        let revolve2 = shader.compute("revolve2").layout(layout).make()?;
        let basis = shader.compute("basis").layout(layout).make()?;
        let nurbs = shader.compute("nurbs").layout(layout).make()?;
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
    pub travel: Grc<ComputePipeline>,
    pub orient: Grc<ComputePipeline>,
    pub spline: Grc<ComputePipeline>,
}

impl WeaveGridPlotBin {
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/weave.wgsl"));
        let travel = shader.compute("travel").layout(layout).make()?;
        let orient = shader.compute("orient").layout(layout).make()?;
        let spline = shader.compute("spline").layout(layout).make()?;
        Ok(Self {
            travel,
            orient,
            spline,
        })
    }
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
            .pipe_layout(&[&store.topic.layout_vertex, &store.rig.layout])
            .make()?;
        Ok(Self {
            chart: draw::ChartBank::new(port, &layout)?,
        })
    }
}
