use super::*;

pub struct Part<'a> {
    pub rig: &'a Charter<'a>,
    pub buffer: &'a Hub<Grc<Buffer>>,
}

impl Part<'_> {
    pub fn nurbs(&self, rig: &Hedge, span: &Hedge) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.rig.plot.core.gpu;
        let nurbs = &self.rig.plot.core.bank.plot.grid.basis.nurbs;
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
            .dispatch(self.rig.count.clone())
            .hub()
    }
}