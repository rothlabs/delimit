use super::*;

#[derive(Debug)]
pub struct ChartBank {
    pub points: RenderProgram,
}

impl ChartBank {
    pub fn new(port: &Viewport, layout: &PipelineLayout) -> Result<Self> {
        let gpu = &port.gpu;
        let shader = port.shader(include_wgsl!("draw/points.wgsl"));
        let attribs = vertex_attr_array![0 => Float32x2];
        let buffers = vec![gpu.vertex_layout(8).attributes(&attribs).make()?];
        let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
        let fragment = shader.fragment("fs_main").make()?;
        let multi = gpu.multisample(4).make()?;
        let pipe = port
            .pipe(vertex)
            .fragment(fragment)
            .layout(layout)
            .multisample(multi)
            .make()?;
        let points = RenderProgram { pipe };
        Ok(Self { points })
    }
}
