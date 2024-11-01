use super::*;

pub struct Part<'a> {
    pub charter: &'a Charter<'a>,
    pub weft: &'a Hub<Grc<Buffer>>,
}

impl Part<'_> {
    pub fn nurbs(&self, rig: &Hedge, arch: &Hedge) -> graph::Result<Hub<Mutation>> {
        let core = &self.charter.plot.core;
        let gpu = &core.gpu;
        let nurbs = &core.bank.plot.grid.basis.nurbs;
        let bind = gpu
            .bind()
            .layout(nurbs.layout.clone())
            .entry(0, rig.buffer.clone())
            .entry(1, arch.buffer.clone())
            .entry(2, self.weft.clone())
            .hub()?;
        gpu.command()
            .root(rig.root.clone())
            .root(arch.root.clone())
            .compute(nurbs.pipe.clone())
            .bind(0, bind)
            .dispatch(self.charter.count.clone())
            .hub()
    }
}