use super::*;

#[derive(Debug)]
pub struct Chart {
    pub points: Grc<RenderPipeline>,
}

impl Chart {
    pub fn new(port: &Viewport, layout: &PipelineLayout) -> Result<Self> {
        let gpu = &port.gpu;
        let shader = port.shader(include_wgsl!("image/points.wgsl"));
        // let vertex = shader.vertex("vs_main").make()?;

        let fragment = shader.fragment("fs_main").make()?;
        let multi = gpu.multisample(4).make()?;
        let points = port
            .pipe(vertex)
            .fragment(fragment)
            .layout(layout)
            .multisample(multi)
            .make()?;
        Ok(Self { points })
    }
}

struct Shader<'a> {
    device: &'a Device,
    layout: &'a PipelineLayout,
    module: ShaderModule,
}

impl Shader<'_> {
    fn pipe(&self, entry_point: &str) -> Grc<ComputePipeline> {
        let vertex = VertexState {
            module: &self.module,
            entry_point,
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        };
        self.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(entry_point),
            layout: Some(&self.layout),
            vertex,
            fragment: built.fragment,
            primitive: built.primitive,
            depth_stencil: built.depth_stencil,
            multisample: built.multisample,
            multiview: built.multiview,
            cache: built.cache,
        }).into()
    }
}



// impl Chart {
//     pub fn new(port: &Viewport, layout: &PipelineLayout) -> Result<Self> {
//         let gpu = &port.gpu;
//         let shader = port.shader(include_wgsl!("image/points.wgsl"));
//         let vertex = shader.vertex("vs_main").make()?; 
//         let fragment = shader.fragment("fs_main").make()?;
//         let multi = gpu.multisample(4).make()?;
//         let points = port
//             .pipe(vertex)
//             .fragment(fragment)
//             .layout(layout)
//             .multisample(multi)
//             .make()?;
//         Ok(Self { points })
//     }
// }







// .buffers(&buffers)

// let attribs = vertex_attr_array![0 => Float32x2];
        // let buffers = vec![gpu.vertex_layout(8).attributes(&attribs).make()?];

// let buffers = vec![gpu.vertex_layout(8).attributes(&attribs).make()?];
// let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
