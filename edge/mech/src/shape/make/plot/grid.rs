use super::*;

pub struct Form<'a> {
    pub grid: &'a Grid<'a>,
    pub buffer: &'a Hub<Grc<Buffer>>,
}

impl Form<'_> {
    pub fn nurbs(&self, rig: &Hedge, span: &Hedge) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.grid.plot.core.gpu;
        let nurbs = &self.grid.plot.core.bank.plot.grid.basis.nurbs;
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
            .dispatch(self.grid.count.clone())
            .hub()
    }
}

pub struct Control<'a> {
    pub grid: &'a Grid<'a>,
    pub basis: &'a Basis,
}

impl Control<'_> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        panic!("wow")
        // self.grid.plot.shape.control
    }
}