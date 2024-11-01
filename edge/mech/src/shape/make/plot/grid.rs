use super::*;

pub struct Form<'a> {
    pub array: &'a Array<'a>,
    pub buffer: &'a Hub<Grc<Buffer>>,
}

impl Form<'_> {
    pub fn nurbs(&self, rig: &Hedge, span: &Hedge) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.array.plot.core.gpu;
        let nurbs = &self.array.plot.core.bank.plot.grid.basis.nurbs;
        let bind = gpu
            .bind()
            .layout(nurbs.layout.clone())
            .entry(0, rig.buffer.clone())
            .entry(1, span.buffer.clone())
            .entry(2, self.buffer.clone())
            .hub()?;
        gpu.command()
            .root(rig.root.clone())
            .root(span.root.clone())
            .compute(nurbs.pipe.clone())
            .bind(0, bind)
            .dispatch(self.array.count.clone())
            .hub()
    }
}

pub struct Control<'a> {
    pub grid: &'a Grid<'a>,
    // per rank
    pub basis: Vec<Basis>,
    // per rank
    pub strides: Vec<Hub<u32>>,
}

impl<'a> Control<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let shape = &self.grid.plot.shape;
        let counts = self.grid.counts;
        let mut plots = vec![shape.points.clone()];
        for rank in 0..shape.index.len() {
            let plot = plots.last().ok_or(anyhow!("no plot"))?;
            plots.push(self.step(rank, plot)?);
        }
        plots.last().cloned().ok_or(Err(anyhow!("no plot"))?)
    }
    fn step(&self, rank: usize, plot: &'a Hedge) -> graph::Result<Hedge> {
        Step {
            control: self,
            rank,
            plot,
        }
        .hedge()
    }
}

struct Step<'a> {
    control: &'a Control<'a>,
    rank: usize,
    plot: &'a Hedge,
}

impl Step<'_> {
    fn hedge(&self) -> graph::Result<Hedge> {
        let gpu = &self.control.grid.plot.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let buffer = gpu.blank(size).hub()?;
        let mut root = JoinBuilder::default();
        let basis = self.basis()?;
        for (order, vector) in basis.vector.iter().enumerate() {
            if let Some(span) = vector {
                // TODO: order + 1 to account for basis.matrix 
                let offset = offsets.get(order).ok_or(anyhow!("no offset"))?;
                let rig = self.rig(order, offset)?;
                
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
        let basis = self.basis()?;
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
    fn rig(&self, order: usize, offset: &Hub<u32>) -> graph::Result<Hedge> {
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
    fn basis(&self) -> graph::Result<&Basis> {
        let basis = &self.control.basis;
        let last = basis.last().ok_or(anyhow!("no basis"))?;
        Ok(basis.get(self.rank).unwrap_or(last))
    }
    fn stride(&self) -> graph::Result<&Hub<u32>> {
        let stride = self.control.strides.get(self.rank);
        stride.ok_or(Err(anyhow!("no stride"))?)
    }
}

// fn step(&self, i: usize, plot: &'a Hedge) -> graph::Result<Step> {
//     let last_basis = self.basis.last().ok_or(anyhow!("no basis"))?;
//     let index = &self.grid.plot.shape.index;
//     Ok(Step {
//         grid: &self.grid,
//         plot,
//         basis: self.basis.get(i).unwrap_or(last_basis),
//         index: index.get(i).ok_or(anyhow!("no index"))?,
//     })
// }

// if counts.len() > 1 {
//     let mut stride = counts[0].calc();
//     for count in counts.iter().skip(1) {
//         stride = stride.mul(count);
//     }
//     stride.hub()
// } else if counts.len() > 0 {
//     Ok(counts[0].clone())
// } else {
//     Ok(1.into())
// }
