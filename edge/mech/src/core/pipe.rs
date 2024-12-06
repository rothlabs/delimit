use super::*;

mod chart;
mod image;

#[derive(Debug)]
pub struct Chart {
    pub grid: chart::Grid,
}

impl Chart {
    pub fn new(gpu: &Gpu) -> Result<Self> {
        let store = &gpu.store;
        let layout = gpu.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&store.topic.layout, &store.rig.layout],
            push_constant_ranges: &[],
        });
        Ok(Self {
            grid: chart::Grid::new(gpu, &layout)?,
        })
    }
}

#[derive(Debug)]
pub struct Image {
    pub chart: image::Chart,
}

impl Image {
    pub fn new(port: &Viewport) -> Result<Self> {
        let store = &port.gpu.store;
        let layout = port.gpu.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&store.topic.layout_vertex, &store.rig.layout],
            push_constant_ranges: &[],
        });
        Ok(Self {
            chart: image::Chart::new(port, &layout)?,
        })
    }
}

// let layout = port
//             .gpu
//             .pipe_layout(&[&store.topic.layout_vertex, &store.rig.layout])
//             .make()?;
