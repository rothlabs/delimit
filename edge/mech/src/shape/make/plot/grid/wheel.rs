use super::*;

pub struct Spin<'a> {
    pub charter: &'a Wheel<'a>,
    pub weft: &'a Hub<Grc<Buffer>>,
}

impl Spin<'_> {
    pub fn nurbs(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Mutation>> {
        let core = &self.charter.plot.core;
        let gpu = &core.gpu;
        let nurbs = &core.bank.plot.grid.right.nurbs;
        let bind = gpu
            .bind()
            .layout(nurbs.layout.clone())
            .entry(0, &rig.buffer)
            .entry(1, &form.buffer)
            .entry(2, self.weft)
            .hub()?;
        gpu.command()
            .root(&rig.root)
            .root(&form.root)
            .compute(nurbs.pipe.clone())
            .bind(0, bind)
            .dispatch(self.charter.count)
            .hub()
    }
}
