use super::*;

mod mesh;

#[derive(Debug)]
pub struct Bank {
    pub chart: ChartBank,
}

impl Bank {
    pub fn new(port: &Viewport) -> Result<Self> {
        Ok(Self {
            chart: ChartBank::new(port)?,
        })
    }
}

#[derive(Debug)]
pub struct ChartBank {
    pub points: RenderMeshProgram,
}

impl ChartBank {
    fn new(port: &Viewport) -> Result<Self> {
        let shader = port.shader(include_wgsl!("plot/points.wgsl"));
        // let targets = port.display.targets();
        let rig = port.gpu.bind_uniform().entry(0)?.vertex()?;
        let plot = port.gpu.bind_storage(true).entry(1)?.vertex()?;
        let layout = port.gpu.bind_layout(&[rig, plot]).make()?;
        // TODO: put pipe_layout method on bind_layout
        let pipe_layout = port.gpu.pipe_layout(&[&layout]).make()?;
        let attribs = vertex_attr_array![0 => Float32x2];
        let buffers = vec![port.gpu.vertex_layout(8).attributes(&attribs).make()?];
        let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
        let fragment = shader.fragment("fs_main").make()?; // .targets(targets)
        let multi = port.gpu.multisample(4).make()?;
        let pipe = port
            .pipe(vertex)
            .fragment(fragment)
            .layout(&pipe_layout)
            .multisample(multi)
            .make()?;
        let count: u32 = 8;
        let points = mesh::Circle {
            frame: port.size.clone(), //gpu.display.config.clone(),
            count: count.into(),
            radius: 4.0.into(),
        }
        .gate()?;
        let buffer = port.gpu.buffer(count as u64 * 24).vertex()?;
        let mesh = Hedge {
            root: port.gpu.writer(buffer.clone()).data(points).hub()?,
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

// #[derive(Debug)]
// pub struct ComputeProgram {
//     pub layout: Grc<BindGroupLayout>,
//     pub pipe: Grc<ComputePipeline>,
// }

#[derive(Debug)]
pub struct RenderMeshProgram {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<RenderPipeline>,
    pub mesh: Hedge,
    pub vertex_count: u32,
}
