use super::*;

pub struct Stage<'a> {
    pub control: &'a Control<'a>,
    pub rank: usize,
}

impl<'a> Stage<'a> {
    pub fn hedge(&self, warp: &Hedge) -> graph::Result<Hedge> {
        let gpu = &self.control.grid.plot.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let buffer = gpu.blank(size).hub()?;
        let part = self.part(warp, &buffer)?;
        let mut root = JoinBuilder::default();
        let jamb = self.jamb()?;
        let weft = self.weft()?;
        for (order, jamb) in jamb.matrix.iter().enumerate() {
            if let Some(jamb) = jamb {
                let pair = Pair {
                    weft: weft.vector(order)?,
                    jamb,
                };
                // TODO: order + 1 to account for weft.matrix
                let offset = offsets.get(order).ok_or(anyhow!("no offset"))?;
                let rig = self.matrix_rig(order, offset)?;
                root.field(part.matrix(&rig, &pair)?);
            }
        }
        let root = root.hub()?;
        Ok(Hedge { buffer, root })
    }
    fn offsets(&self) -> graph::Result<Vec<Hub<u32>>> {
        let gpu = &self.control.grid.plot.core.gpu;
        let dimension = self.control.grid.plot.shape.dimension;
        let jamb = self.jamb()?;
        let mut offsets: Vec<Hub<u32>> = vec![0.into()];
        // if let Some(weft) = &weft.matrix {
        //     // TODO: mul div sub weft_size as needed
        //     let size_part = gpu.size(&weft.buffer).hub()?;
        //     size = size.add(size_part);
        // }
        for (order, jamb) in jamb.matrix.iter().enumerate() {
            if let Some(jamb) = jamb {
                let builder = gpu.size(&jamb.buffer).div(order as u32 + 1).mul(dimension);
                let size = builder.mul(self.count()?).mul(self.stride()?).hub()?;
                // TODO: (rank + 1) * 2 when acceleration is included
                let size = size.calc().mul(self.rank as u32 + 1).add(&size).hub()?;
                let offset = offsets.last().ok_or(anyhow!("no offsets"))?.calc();
                offsets.push(offset.add(size).hub()?);
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
            .field(order as u32)
            .field(self.count()?)
            .field(self.stride()?)
            .field(dimension)
            .field(offset)
            .make()
    }
    fn jamb(&self) -> graph::Result<&Jamb> {
        let jambs = &self.control.grid.plot.shape.jambs;
        jambs.get(self.rank).ok_or(Err(anyhow!("no jamb"))?)
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
        stride.ok_or(Err(anyhow!("no stride"))?)
    }
    fn part(&self, warp: &'a Hedge, plot: &'a Hub<Grc<Buffer>>) -> graph::Result<Part> {
        Ok(Part {
            stage: self,
            warp,
            plot,
            count: self.count()?,
        })
    }
}

struct Pair<'a> {
    weft: &'a Hedge,
    jamb: &'a Hedge,
}

struct Part<'a> {
    stage: &'a Stage<'a>,
    warp: &'a Hedge,
    plot: &'a Hub<Grc<Buffer>>,
    count: &'a Hub<u32>,
}

impl Part<'_> {
    fn matrix(&self, rig: &Hedge, pair: &Pair) -> graph::Result<Hub<Mutation>> {
        let core = &self.stage.control.grid.plot.core;
        let gpu = &core.gpu;
        let control = &core.bank.plot.grid.basis.control;
        let bind = gpu
            .bind()
            .layout(control.layout.clone())
            .entry(0, &rig.buffer)
            .entry(1, &self.warp.buffer)
            .entry(2, &pair.weft.buffer)
            .entry(3, &pair.jamb.buffer)
            .entry(4, self.plot)
            .hub()?;
        gpu.command()
            .root(&rig.root)
            .root(&self.warp.root)
            .root(&pair.weft.root)
            .root(&pair.jamb.root)
            .compute(control.pipe.clone())
            .bind(0, bind)
            .dispatch(self.count)
            .hub()
    }
}
