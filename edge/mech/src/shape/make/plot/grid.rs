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
}

impl<'a> Control<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let indices = &self.grid.plot.shape.index;
        // for index in indices.iter().enumerate() {

        // }
        panic!("wow")
        // self.grid.plot.shape.control
    }
    fn step(&self, i: usize, plot: &'a Hedge) -> graph::Result<Step> {
        let last_basis = self.basis.last().ok_or(anyhow!("no basis"))?;
        let index = &self.grid.plot.shape.index;
        Ok(Step {
            grid: &self.grid,
            plot,
            basis: self.basis.get(i).unwrap_or(last_basis),
            index: index.get(i).ok_or(anyhow!("no index"))?,
        })
    }
}

struct Step<'a> {
    // rank: u32,
    grid: &'a Grid<'a>,
    plot: &'a Hedge,
    basis: &'a Basis,
    index: &'a Index,
}
