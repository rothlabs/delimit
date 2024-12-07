use super::*;

#[derive(Debug)]
pub struct Chart {
    pub points: Grc<RenderPipeline>,
}

impl Chart {
    pub fn new(form: &Form) -> Self {
        let shader = Shader {
            device: form.device,
            layout: &form.layout.chart,
            target: form.target,
            module: form
                .device
                .create_shader_module(include_wgsl!("chart/points.wgsl")),
        };
        Self {
            points: shader.pipe("vs_main", "fs_main"),
        }
    }
}

struct Shader<'a> {
    device: &'a Device,
    layout: &'a PipelineLayout,
    target: &'a [Option<ColorTargetState>],
    module: ShaderModule,
}

impl Shader<'_> {
    fn pipe(&self, vert_point: &str, frag_entry: &str) -> Grc<RenderPipeline> {
        let vertex = VertexState {
            module: &self.module,
            entry_point: vert_point,
            compilation_options: PipelineCompilationOptions::default(),
            buffers: &[],
        };
        let fragment = FragmentState {
            module: &self.module,
            entry_point: frag_entry,
            compilation_options: PipelineCompilationOptions::default(),
            targets: self.target,
        };
        let multisample = MultisampleState {
            count: 4,
            mask: 0,
            alpha_to_coverage_enabled: false,
        };
        self.device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some(vert_point),
                layout: Some(&self.layout),
                vertex,
                fragment: Some(fragment),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample,
                multiview: None,
                cache: None,
            })
            .into()
    }
}

// let gpu = &port.gpu;
//         let shader = port.shader(include_wgsl!("chart/points.wgsl"));
//         // let vertex = shader.vertex("vs_main").make()?;

//         let fragment = shader.fragment("fs_main").make()?;
//         let multi = gpu.multisample(4).make()?;
//         let points = port
//             .pipe(vertex)
//             .fragment(fragment)
//             .layout(layout)
//             .multisample(multi)
//             .make()?;
//         Ok(Self { points })
