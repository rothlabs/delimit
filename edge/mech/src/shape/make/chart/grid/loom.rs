use super::*;

pub struct Trio<'a> {
    pub rig: Hedge,
    pub weft: &'a Hedge,
    pub flow: &'a Hedge,
}

pub struct Weave<'a> {
    pub loom: &'a Loom<'a>,
    pub warp: &'a Hedge,
    pub plot: &'a Hub<Grc<Buffer>>,
    pub count: &'a Hub<u32>,
}

impl Weave<'_> {
    pub fn travel(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.travel;
        self.weave(trio, program)
    }
    pub fn orient(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.orient;
        self.weave(trio, program)
    }
    pub fn spline(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.spline;
        self.weave(trio, program)
    }
    pub fn weave(&self, trio: Trio, program: &ComputeProgram) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.loom.grid.chart.core.gpu;
        let bind = gpu
            .bind()
            .layout(program.layout.clone())
            .entry(0, &trio.rig.buffer)
            .entry(1, &self.warp.buffer)
            .entry(2, &trio.weft.buffer)
            .entry(3, &trio.flow.buffer)
            .entry(4, self.plot)
            .hub()?;
        gpu.compute()
            .root(&trio.rig.root)
            .root(&self.warp.root)
            .root(&trio.weft.root)
            .root(&trio.flow.root)
            .pipe(program.pipe.clone())
            .bind(0, bind)
            .dispatch(self.count)
            .hub()
    }
}
