use super::*;

pub struct Spin<'a> {
    pub charter: &'a Wheel<'a>,
    pub weft: &'a Hub<Grc<Buffer>>,
}

impl Spin<'_> {
    pub fn spline(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Mutation>> {
        let program = &self.charter.plot.core.bank.plot.grid.right.spline;
        self.spline_or_nurbs(rig, form, program)
    }
    pub fn nurbs(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Mutation>> {
        let program = &self.charter.plot.core.bank.plot.grid.right.nurbs;
        self.spline_or_nurbs(rig, form, program)
    }
    fn spline_or_nurbs(
        &self,
        rig: &Hedge,
        form: &Hedge,
        program: &ComputeProgram,
    ) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.charter.plot.core.gpu;
        let bind = gpu
            .bind()
            .layout(program.layout.clone())
            .entry(0, &rig.buffer)
            .entry(1, &form.buffer)
            .entry(2, self.weft)
            .hub()?;
        gpu.command()
            .root(&rig.root)
            .root(&form.root)
            .compute(program.pipe.clone())
            .bind(0, bind)
            .dispatch(self.charter.count)
            .hub()
    }
}
