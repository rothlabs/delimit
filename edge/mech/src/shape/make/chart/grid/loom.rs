use super::*;

pub struct Trio<'a> {
    pub rig: Hedge,
    pub weft: &'a Hedge,
    pub flow: &'a Hedge,
}

pub struct Weave<'a> {
    pub loom: &'a Loom<'a>,
    pub warp: &'a Hedge,
    // pub plot: &'a Hub<Grc<Buffer>>,
    pub size: Hub<u32>,
}

impl Weave<'_> {
    pub fn travel(&self, trio: Trio) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.travel;
        self.weave(trio, program)
    }
    pub fn orient(&self, trio: Trio) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.orient;
        self.weave(trio, program)
    }
    pub fn spline(&self, trio: Trio) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let core = &self.loom.grid.chart.core;
        let program = &core.bank.plot.grid.weave.spline;
        self.weave(trio, program)
    }
    pub fn weave(
        &self,
        trio: Trio,
        program: &ComputeProgram,
    ) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let gpu_ = &self.loom.grid.chart.core.gpu;
        let uniform = &gpu_.store.uniform.group;
        let storage = &gpu_.store.storage.group;
        // let bind = gpu::bind().slot(1).group(group).hub()?;
        let stems = trio
            .rig
            .stems
            .with(&self.warp.stems)
            .with(&trio.weft.stems)
            .with(&trio.flow.stems);
        // TODO: replace with a mech fn that already has the pipe and Hub<Bind>
        gpu::dispatch()
            .pipe(&program.pipe)
            .bind(gpu::bind().slot(0).group(storage).hub()?)
            .bind(gpu::bind().slot(1).group(uniform).hub()?)
            .size(&self.size)
            .stems(stems)
            .hub()
    }
}

// TODO make func that creates this structure from ComputeProgram, Hedges, and count
// let bind = gpu_
//     .bind()
//     .layout(program.layout.clone())
//     .entry(0, &trio.rig.buffer)
//     .entry(1, &self.warp.buffer)
//     .entry(2, &trio.weft.buffer)
//     .entry(3, &trio.flow.buffer)
//     .entry(4, self.plot)
//     .hub()?;
