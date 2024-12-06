use super::*;

#[derive(Debug)]
pub struct Grid {
    pub spin: SpinGrid,
    pub weave: WeaveGrid,
}

impl Grid {
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        Ok(Self {
            spin: SpinGrid::new(gpu, layout)?,
            weave: WeaveGrid::new(gpu, layout)?,
        })
    }
}

#[derive(Debug)]
pub struct SpinGrid {
    // travel
    pub extrude: Grc<ComputePipeline>,

    // orient
    pub revolve2: Grc<ComputePipeline>,

    // spline
    pub basis: Grc<ComputePipeline>,
    pub nurbs: Grc<ComputePipeline>,
}

impl SpinGrid {
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("chart/grid/spin.wgsl"));
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
pub struct WeaveGrid {
    pub travel: Grc<ComputePipeline>,
    pub orient: Grc<ComputePipeline>,
    pub spline: Grc<ComputePipeline>,
}

impl WeaveGrid {
    pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("chart/grid/weave.wgsl"));
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