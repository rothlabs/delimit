use super::*;

pub struct Trio<'a> {
    pub rig: Hedge,
    pub weft: &'a Hedge,
    pub flow: &'a Hedge,
}

pub struct Part<'a> {
    pub stage: &'a Weave<'a>,
    pub warp: &'a Hedge,
    pub plot: &'a Hub<Grc<Buffer>>,
    pub count: &'a Hub<u32>,
}

impl Part<'_> {
    pub fn travel(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.stage.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.travel;
        self.weave(trio, program)
    }
    pub fn orient(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.stage.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.orient;
        self.weave(trio, program)
    }
    pub fn spline(&self, trio: Trio) -> graph::Result<Hub<Mutation>> {
        let core = &self.stage.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.spline;
        self.weave(trio, program)
    }
    pub fn weave(&self, trio: Trio, program: &ComputeProgram) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.stage.loom.grid.chart.core.gpu;
        let bind = gpu
            .bind()
            .layout(program.layout.clone())
            .entry(0, &trio.rig.buffer)
            .entry(1, &self.warp.buffer)
            .entry(2, &trio.weft.buffer)
            .entry(3, &trio.flow.buffer)
            .entry(4, self.plot)
            .hub()?;
        gpu.command()
            .root(&trio.rig.root)
            .root(&self.warp.root)
            .root(&trio.weft.root)
            .root(&trio.flow.root)
            .compute(program.pipe.clone())
            .bind(0, bind)
            .dispatch(self.count)
            .hub()
    }
}
