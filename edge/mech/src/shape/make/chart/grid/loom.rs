use super::*;

pub struct Trio<'a> {
    pub rig: Hedge,
    pub weft: &'a Hedge,
    pub flow: &'a Hedge,
}

pub struct Weave<'a> {
    pub loom: &'a Loom<'a>,
    pub warp: &'a Hedge,
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
        pipe: &Grc<ComputePipeline>,
    ) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let gpu = &self.loom.grid.chart.core.gpu;
        let rig_group = &gpu.store.rig.group;
        // let storage = &gpu_.store.topic.group;
        let rig_bind = gpu::Bind::builder()
            .slot(1)
            .group(rig_group)
            .offsets(vec![trio.rig.offset])
            .build()
            .hub();
        let stems = trio
            .rig
            .stems
            .with(&self.warp.stems)
            .with(&trio.weft.stems)
            .with(&trio.flow.stems);
        // TODO: replace with a mech fn that already has the pipe and Hub<Bind>
        gpu::dispatch()
            .pipe(pipe)
            .bind(&gpu.store.topic.bind)
            .bind(rig_bind)
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
