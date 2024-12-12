use super::*;

#[derive(Debug)]
pub struct Grid {
    pub spin: SpinGrid,
    pub weave: WeaveGrid,
}

impl Grid {
    pub(super) fn new(layout: &PipeLayout) -> Self {
        Self {
            spin: SpinGrid::new(layout),
            weave: WeaveGrid::new(layout),
        }
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
    pub(super) fn new(pipe: &PipeLayout) -> Self {
        let shader = Shader {
            device: pipe.device,
            layout: &pipe.layout,
            module: pipe
                .device
                .create_shader_module(include_wgsl!("chart/grid/spin.wgsl")),
        };
        Self {
            extrude: shader.pipe("extrude"),
            revolve2: shader.pipe("revolve2"),
            basis: shader.pipe("basis"),
            nurbs: shader.pipe("nurbs"),
        }
    }
}

#[derive(Debug)]
pub struct WeaveGrid {
    pub travel: Grc<ComputePipeline>,
    pub orient: Grc<ComputePipeline>,
    pub spline: Grc<ComputePipeline>,
}

impl WeaveGrid {
    pub(super) fn new(pipe: &PipeLayout) -> Self {
        let shader = Shader {
            device: pipe.device,
            layout: &pipe.layout,
            module: pipe
                .device
                .create_shader_module(include_wgsl!("chart/grid/weave.wgsl")),
        };
        Self {
            travel: shader.pipe("travel"),
            orient: shader.pipe("orient"),
            spline: shader.pipe("spline"),
        }
    }
}

struct Shader<'a> {
    device: &'a Device,
    layout: &'a PipelineLayout,
    module: ShaderModule,
}

impl Shader<'_> {
    fn pipe(&self, entry_point: &str) -> Grc<ComputePipeline> {
        self.device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(self.layout),
                module: &self.module,
                entry_point,
                compilation_options: Default::default(),
                cache: None,
            })
            .into()
    }
}

// impl SpinGrid {
//     pub fn new(layout: &Layout) -> Result<Self> {
//         let shader = gpu.shader(include_wgsl!("chart/grid/spin.wgsl"));
//         let extrude = shader.compute("extrude").layout(layout).make()?;
//         let revolve2 = shader.compute("revolve2").layout(layout).make()?;
//         let basis = shader.compute("basis").layout(layout).make()?;
//         let nurbs = shader.compute("nurbs").layout(layout).make()?;
//         Ok(Self {
//             extrude,
//             revolve2,
//             basis,
//             nurbs,
//         })
//     }
// }

// #[derive(Debug)]
// pub struct WeaveGrid {
//     pub travel: Grc<ComputePipeline>,
//     pub orient: Grc<ComputePipeline>,
//     pub spline: Grc<ComputePipeline>,
// }

// impl WeaveGrid {
//     pub fn new(gpu: &Gpu, layout: &PipelineLayout) -> Result<Self> {
//         let shader = gpu.shader(include_wgsl!("chart/grid/weave.wgsl"));
//         let travel = shader.compute("travel").layout(layout).make()?;
//         let orient = shader.compute("orient").layout(layout).make()?;
//         let spline = shader.compute("spline").layout(layout).make()?;
//         Ok(Self {
//             travel,
//             orient,
//             spline,
//         })
//     }
// }
