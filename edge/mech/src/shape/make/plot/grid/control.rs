use super::*;

pub struct Stage<'a> {
    pub control: &'a Control<'a>,
    pub rank: usize,
    pub plot: &'a Hedge,
}

impl Stage<'_> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let gpu = &self.control.grid.plot.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let buffer = gpu.blank(size).hub()?;
        let mut root = JoinBuilder::default();
        let span = self.span()?;
        for (order, vector) in span.vector.iter().enumerate() {
            if let Some(hedge) = vector {
                // TODO: order + 1 to account for basis.matrix
                let offset = offsets.get(order).ok_or(anyhow!("no offset"))?;
                let rig = self.matrix_rig(order, offset)?;
            }
        }
        // let indices = &self.control.grid.plot.shape.index;
        // let index = indices.get(self.rank).ok_or(anyhow!("no index"))?;
        let root = root.hub()?;
        Ok(Hedge { buffer, root })
    }
    // make list of offsets and use last to make buffer
    fn offsets(&self) -> graph::Result<Vec<Hub<u32>>> {
        let gpu = &self.control.grid.plot.core.gpu;
        let dimension = self.control.grid.plot.shape.dimension;
        let basis = self.span()?;
        let mut offsets: Vec<Hub<u32>> = vec![0.into()];
        // if let Some(span) = &basis.matrix {
        //     // TODO: mul div sub span_size as needed
        //     let size_part = gpu.size(&span.buffer).hub()?;
        //     size = size.add(size_part);
        // }
        for (order, vector) in basis.vector.iter().enumerate() {
            if let Some(span) = vector {
                let builder = gpu.size(&span.buffer).mul(dimension);
                let size = builder.mul(self.stride()?).div(order as u32).hub()?;
                let offset = offsets.last().ok_or(anyhow!("no offsets"))?.calc();
                offsets.push(offset.add(size).hub()?);
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
    fn count(&self) -> graph::Result<&Hub<u32>> {
        let counts = &self.control.grid.counts;
        let last = counts.last().ok_or(anyhow!("no counts"))?;
        Ok(counts.get(self.rank).unwrap_or(last))
    }
    fn span(&self) -> graph::Result<&Span> {
        let spans = &self.control.spans;
        let last = spans.last().ok_or(anyhow!("no basis"))?;
        Ok(spans.get(self.rank).unwrap_or(last))
    }
    fn stride(&self) -> graph::Result<&Hub<u32>> {
        let stride = self.control.strides.get(self.rank);
        stride.ok_or(Err(anyhow!("no stride"))?)
    }
}

struct Part<'a> {
    stage: &'a Stage<'a>,    
    jamb: &'a Hedge,
    // span: &'a Hedge,
}

// impl Part<'_> {
//     fn matrix(&self, rig: &Hedge, span: &Hedge) -> graph::Result<Hub<Mutation>> {
//         let core = &self.stage.control.grid.plot.core;
//         let gpu = &core.gpu;
//         let control = &core.bank.plot.grid.basis.control;
//         let bind = gpu
//             .bind()
//             .layout(control.layout.clone())
//             .entry(0, rig.buffer)
//             .entry(1, span.buffer.clone())
//             .entry(2, self.shape.index.buffer.clone())
//             .entry(3, control.buffer.clone())
//             .entry(4, buffer.clone())
//             .hub()?;
//         gpu
//             .command()
//             .root(rig.root)
//             .root(self.hedge.root.clone())
//             .root(self.shape.index.root.clone())
//             .root(control.root.clone())
//             .compute(control.pipe.clone())
//             .bind(0, bind)
//             .dispatch(self.count.clone())
//             .hub()
//     }
// }