use super::*;

mod chart;
mod image;

struct PipeLayout<'a> {
    device: &'a Device,
    layout: PipelineLayout,
}

#[derive(Debug)]
pub struct Chart {
    pub grid: chart::Grid,
}

impl Chart {
    pub fn new(layout: &GroupLayout) -> Self {
        let layout = PipeLayout{
            device: layout.device,
            layout: layout.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&layout.topic, &layout.rig],
            push_constant_ranges: &[],
        })};
        Self {
            grid: chart::Grid::new(&layout),
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub chart: image::Chart,
}

impl Image {
    pub fn new(layout: &GroupLayout) -> Result<Self> {
        let layout = layout.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&layout.image, &layout.rig],
            push_constant_ranges: &[],
        });
        Ok(Self {
            chart: image::Chart::new(layout)?,
        })
    }
}

// let layout = port
//             .gpu
//             .pipe_layout(&[&store.topic.layout_vertex, &store.rig.layout])
//             .make()?;
