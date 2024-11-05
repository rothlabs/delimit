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
        let mut offset_idx = 0;
        if let Some(flow) = &flow.add {
            offset_idx += 1;
        }
        for (order, flow) in flow.left.iter().enumerate() {
            if let Some(flow) = flow {
                let offset = offsets.get(offset_idx).ok_or(anyhow!("no offset"))?;
                root.field(part.right(weave::Trio {
                    rig: self.right_rig(order, offset)?,
                    weft: weft.right(order)?,
                    flow,
                })?);
                offset_idx += 1;
            }
        }
        let root = root.hub()?;
        Ok(Hedge { buffer, root })
    }
    fn offsets(&self) -> graph::Result<Vec<Hub<u32>>> {
        let chart = &self.loom.grid.chart;
        let dimension = chart.shape.dimension;
        let gpu = &chart.core.gpu;
        let flow = self.flow()?;
        let constant = dimension * (self.rank as u32 + 2);
        let expand = self.count()?.calc().mul(self.area()?).mul(constant).hub()?;
        let mut offsets: Vec<Hub<u32>> = vec![0.into()];
        if let Some(flow) = &flow.add {
            let size = gpu.size(&flow.buffer).div(2).mul(&expand).hub()?;
            offsets.push(size);
        }
        for (order, flow) in flow.left.iter().enumerate() {
            if let Some(flow) = flow {
                let size = gpu.size(&flow.buffer).div(order as u32 + 1).hub()?;
                let last = offsets.last().ok_or(anyhow!("no offsets"))?;
                offsets.push(size.calc().mul(&expand).add(last).hub()?);
            }
        }
        Ok(offsets)
    }
    fn right_rig(&self, order: usize, offset: &Hub<u32>) -> graph::Result<Hedge> {
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
