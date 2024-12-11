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
    pub fn travel(&self, trio: Trio) -> Hub<Grc<gpu::Action>> {
        let core = &self.loom.grid.mech;
        let program = &core.pipe.chart.grid.weave.travel;
        self.weave(trio, program)
    }
    pub fn orient(&self, trio: Trio) -> Hub<Grc<gpu::Action>> {
        let core = &self.loom.grid.mech;
        let program = &core.pipe.chart.grid.weave.orient;
        self.weave(trio, program)
    }
    pub fn spline(&self, trio: Trio) -> Hub<Grc<gpu::Action>> {
        let core = &self.loom.grid.mech;
        let program = &core.pipe.chart.grid.weave.spline;
        self.weave(trio, program)
    }
    pub fn weave(&self, trio: Trio, pipe: &Grc<ComputePipeline>) -> Hub<Grc<gpu::Action>> {
        let mech = &self.loom.grid.mech;
        let rig_bind = gpu::active::group::Bind {
            slot: 1.into(),
            group: mech.group.rig.clone(),
            offsets: vec![trio.rig.index],
        };
        let stems = trio
            .rig
            .stems
            .with(&self.warp.stems)
            .with(&trio.weft.stems)
            .with(&trio.flow.stems);
        // TODO: replace with a mech fn that already has the pipe and Hub<Bind>
        gpu::active::command::dispatch::Direct {
            stems,
            pipe: pipe.into(),
            binds: vec![mech.group.bind.topic.clone(), rig_bind.hub()],
            size: self.size.clone(),
        }
        .hub()
    }
}

// gpu::dispatch()
//             .pipe(pipe)
//             .bind(&mech.group.bind.topic)
//             .bind(rig_bind.hub())
//             .size(&self.size)
//             .stems(stems)
//             .hub()

// let rig_bind = gpu::Bind::builder()
//             .slot(1)
//             .group(rig_group)
//             .offsets(vec![trio.rig.index])
//             .build()
//             .hub();

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
