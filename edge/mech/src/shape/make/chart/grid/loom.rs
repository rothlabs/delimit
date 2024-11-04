use super::*;

mod weave;

pub struct Weave<'a> {
    pub loom: &'a Loom<'a>,
    pub rank: usize,
}

impl<'a> Weave<'a> {
    pub fn hedge(&self, warp: &Hedge) -> graph::Result<Hedge> {
        let gpu = &self.loom.grid.chart.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let label = format!("grid plot rank {}", self.rank);
        let buffer = gpu.blank(size).label(label).hub()?;
        let part = self.part(warp, &buffer)?;
        let mut root = JoinBuilder::default();
        let flow = self.flow()?;
        let weft = self.weft()?;
        for (order, flow) in flow.matrices.iter().enumerate() {
            if let Some(flow) = flow {
                // TODO: order + 1 to account for weft.matrix
                let offset = offsets.get(order).ok_or(anyhow!("no offset"))?;
                root.field(part.right(weave::Trio {
                    rig: self.right(order, offset)?,
                    weft: weft.vector(order)?,
                    flow,
                })?);
            }
        }
        let root = root.hub()?;
        Ok(Hedge { buffer, root })
    }
    fn offsets(&self) -> graph::Result<Vec<Hub<u32>>> {
        let chart = &self.loom.grid.chart;
        let gpu = &chart.core.gpu;
        let flow = self.flow()?;
        let mut offsets: Vec<Hub<u32>> = vec![0.into()];
        // if let Some(weft) = &weft.matrix {
        //     // TODO: mul div sub weft_size as needed
        //     let size_part = gpu.size(&weft.buffer).hub()?;
        //     size = size.add(size_part);
        // }
        for (order, flow) in flow.matrices.iter().enumerate() {
            if let Some(flow) = flow {
                let size = gpu
                    .size(&flow.buffer)
                    .div(order as u32 + 1)
                    .mul(chart.shape.dimension)
                    .hub()?;
                let size = size
                    .calc()
                    .mul(self.rank as u32 + 1)
                    .add(&size)
                    .mul(self.count()?)
                    .mul(self.area()?)
                    .add(offsets.last().ok_or(anyhow!("no offsets"))?)
                    .hub()?;
                offsets.push(size);
            } else {
                offsets.push(0.into());
            }
        }
        Ok(offsets)
    }
    fn right(&self, order: usize, offset: &Hub<u32>) -> graph::Result<Hedge> {
        let dimension = self.loom.grid.chart.shape.dimension;
        let uniform = self.loom.grid.chart.core.gpu.uniform();
        uniform
            .field(self.rank as u32)
            .field(order as u32)
            .field(offset)
            .field(self.count()?)
            .field(self.area()?)
            .field(dimension)
            .make()
    }
    fn flow(&self) -> graph::Result<&Flow> {
        let flows = &self.loom.grid.chart.shape.flows;
        Ok(flows.get(self.rank).ok_or(anyhow!("no flow"))?)
    }
    fn count(&self) -> graph::Result<&Hub<u32>> {
        let counts = &self.loom.grid.counts;
        let last = counts.last().ok_or(anyhow!("no counts"))?;
        Ok(counts.get(self.rank).unwrap_or(last))
    }
    fn weft(&self) -> graph::Result<&Weft> {
        let wefts = &self.loom.wefts;
        let last = wefts.last().ok_or(anyhow!("no weft"))?;
        Ok(wefts.get(self.rank).unwrap_or(last))
    }
    fn area(&self) -> graph::Result<&Hub<u32>> {
        let area = self.loom.areas.get(self.rank);
        Ok(area.ok_or(anyhow!("no area"))?)
    }
    fn part(&self, warp: &'a Hedge, plot: &'a Hub<Grc<Buffer>>) -> graph::Result<weave::Part> {
        Ok(weave::Part {
            stage: self,
            warp,
            plot,
            count: self.count()?,
        })
    }
}
