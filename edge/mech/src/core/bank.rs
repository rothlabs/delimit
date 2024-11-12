use super::*;

mod mesh;

#[derive(Debug)]
pub struct Bank {
    pub plot: PlotBin,
    pub draw: DrawBin,
}

impl Bank {
    pub fn new(gpu: &Gpu) -> Result<Self> {
        Ok(Self {
            plot: PlotBin::new(gpu)?,
            draw: DrawBin::new(gpu)?,
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
    // TODO: rename to weft and make new bin for weave
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
pub struct DrawBin {
    pub points: RenderMeshProgram,
}

impl DrawBin {
    fn new(gpu: &Gpu) -> Result<Self> {
        let shader = gpu.shader(include_wgsl!("draw/points.wgsl"));
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
            radius: 4.0.into(),
            frame: (300, 300).into(),//gpu.display.config.clone(),
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
