use super::*;

mod weave;
// #[cfg(test)]
// mod tests;

pub struct Weave<'a> {
    pub control: &'a Loom<'a>,
    pub rank: usize,
}

impl<'a> Weave<'a> {
    pub fn hedge(&self, warp: &Hedge) -> graph::Result<Hedge> {
        let gpu = &self.control.grid.plot.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let buffer = gpu.blank(size).label(format!("grid plot rank {}", self.rank)).hub()?;
        let part = self.part(warp, &buffer)?;
        let mut root = JoinBuilder::default();
        let flow = self.flow()?;
        let weft = self.weft()?;
        for (order, flow) in flow.matrices.iter().enumerate() {
            if let Some(flow) = flow {
                // TODO: order + 1 to account for weft.matrix
                let offset = offsets.get(order).ok_or(anyhow!("no offset"))?;
                root.field(part.matrix(weave::Trio {
                    rig: self.matrix_rig(order, offset)?,
                    weft: weft.vector(order)?,
                    flow,
                })?);
            }
        }
        let root = root.hub()?;
        Ok(Hedge { buffer, root })
    }
    fn offsets(&self) -> graph::Result<Vec<Hub<u32>>> {
        let gpu = &self.control.grid.plot.core.gpu;
        let dimension = self.control.grid.plot.shape.dimension;
        let flow = self.flow()?;
        let mut offsets: Vec<Hub<u32>> = vec![0.into()];
        // if let Some(weft) = &weft.matrix {
        //     // TODO: mul div sub weft_size as needed
        //     let size_part = gpu.size(&weft.buffer).hub()?;
        //     size = size.add(size_part);
        // }
        for (order, flow) in flow.matrices.iter().enumerate() {
            if let Some(flow) = flow {
                let builder = gpu.size(&flow.buffer).div(3).mul(dimension);//.div(order as u32 + 1).mul(dimension);
                let size = builder.mul(self.count()?).mul(self.stride()?).hub()?;
                // let size = builder.hub()?;
                // panic!("size {}", size.base());
                // TODO: (rank + 1) * 2 when acceleration is included
                let size = size.calc().mul(self.rank as u32 + 1).add(&size).hub()?;
                let offset = offsets.last().ok_or(anyhow!("no offsets"))?.calc();
                offsets.push(offset.add(size).hub()?);
                // panic!("made it ");
            } else {
                offsets.push(0.into());
            }
        }
        Ok(offsets)
    }
    fn matrix_rig(&self, order: usize, offset: &Hub<u32>) -> graph::Result<Hedge> {
        let dimension = self.control.grid.plot.shape.dimension;
        let uniform = self.control.grid.plot.core.gpu.uniform();
        uniform
            .field(self.rank as u32)
            .field(order as u32)
            .field(offset)
            .field(self.count()?)
            .field(self.stride()?)
            .field(dimension)
            .make()
    }
    fn flow(&self) -> graph::Result<&Flow> {
        let flows = &self.control.grid.plot.shape.flows;
        Ok(flows.get(self.rank).ok_or(anyhow!("no flow"))?)
    }
    fn count(&self) -> graph::Result<&Hub<u32>> {
        let counts = &self.control.grid.counts;
        let last = counts.last().ok_or(anyhow!("no counts"))?;
        Ok(counts.get(self.rank).unwrap_or(last))
    }
    fn weft(&self) -> graph::Result<&Weft> {
        let wefts = &self.control.wefts;
        let last = wefts.last().ok_or(anyhow!("no weft"))?;
        Ok(wefts.get(self.rank).unwrap_or(last))
    }
    fn stride(&self) -> graph::Result<&Hub<u32>> {
        let stride = self.control.strides.get(self.rank);
        Ok(stride.ok_or(anyhow!("no stride"))?)
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
