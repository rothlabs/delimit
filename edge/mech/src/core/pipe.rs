use super::*;

mod chart;
// mod image;

struct PipeLayout<'a> {
    device: &'a Device,
    layout: PipelineLayout,
}

#[derive(Debug)]
pub struct Chart {
    pub grid: chart::Grid,
}

impl Chart {
    pub fn new(bank: &group::layout::Bank) -> Self {
        let layout = PipeLayout{
            device: bank.device,
            layout: bank.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&bank.topic, &bank.rig],
            push_constant_ranges: &[],
        })};
        Self {
            grid: chart::Grid::new(&layout),
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub chart: Grc<PipelineLayout>,//image::Chart,
}

impl Image {
    pub fn new(catalog: &group::layout::Bank) -> Self {
        let chart = catalog.device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("mech_chart"),
            bind_group_layouts: &[&catalog.image, &catalog.rig],
            push_constant_ranges: &[],
        }).into();
        Self {
            chart, // image::Chart::new(layout)?,
        }
    }
}

// let layout = port
//             .gpu
//             .pipe_layout(&[&store.topic.layout_vertex, &store.rig.layout])
//             .make()?;
