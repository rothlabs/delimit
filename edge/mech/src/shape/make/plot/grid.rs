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
    pub basis: Vec<Basis>,
    pub strides: Vec<Hub<u32>>,
}

impl<'a> Control<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let shape = &self.grid.plot.shape;
        let counts = self.grid.counts;
        // let last_count = counts.last().ok_or(anyhow!("no counts"))?;
        // let mut stride: Hub<u32> = 1.into();
        let mut plots = vec![shape.points.clone()];
        for rank in 0..shape.index.len() {
            let plot = plots.last().ok_or(anyhow!("no plot"))?;
            plots.push(self.step(rank, plot)?);
            // stride = counts.get(rank).cloned().unwrap_or(last_count.clone());
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
        panic!("wow")
    }
    fn stride(&self) -> graph::Result<Hub<u32>> {
        let counts = self.control.grid.counts;
        if counts.len() > 1 {
            let mut stride = counts[0].calc();
            for count in counts.iter().skip(1) {
                stride = stride.mul(count);
            }
            stride.hub()
        } else if counts.len() > 0 {
            Ok(counts[0].clone())
        } else {
            Ok(1.into())
        }
        // self.control.grid.counts.get(self.rank).ok_or(anyhow!("no count"))?;
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
