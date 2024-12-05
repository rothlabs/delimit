use super::*;

pub struct Spin<'a> {
    pub wheel: &'a Wheel<'a>,
    // pub weft: &'a Hub<Grc<Buffer>>,
    pub size: Hub<u32>,
}

impl Spin<'_> {
    pub fn extrude(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.mech.bank.plot.grid.spin.extrude;
        self.weft(rig, form, program)
    }
    pub fn revolve(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let dimension = self.wheel.shape.dimension;
        if dimension == 2 {
            let program = &self.wheel.mech.bank.plot.grid.spin.revolve2;
            return self.weft(rig, form, program);
        }
        Err(anyhow!(
            "only revolve 2D and 3D supported, found dimension {dimension}"
        ))?
    }
    pub fn basis(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.mech.bank.plot.grid.spin.basis;
        self.weft(rig, form, program)
    }
    pub fn nurbs(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.mech.bank.plot.grid.spin.nurbs;
        self.weft(rig, form, program)
    }
    fn weft(
        &self,
        rig: &Hedge,
        form: &Hedge,
        pipe: &Grc<ComputePipeline>,
    ) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let gpu = &self.wheel.mech.gpu;
        let rig_group = &gpu.store.rig.group;
        let stems = rig.stems.with(&form.stems);
        let rig_bind = gpu::Bind::builder()
            .slot(1)
            .group(rig_group)
            .offsets(vec![rig.index.clone()])
            .build()
            .hub();
        gpu::dispatch()
            .pipe(pipe)
            .bind(&gpu.store.topic.bind)
            .bind(rig_bind)
            .size(&self.size)
            .stems(stems)
            .hub()
    }
}

// let group = gpu_
//     .bind()
//     .layout(program.layout.clone())
//     .entry(0, &rig.buffer)
//     .entry(1, &form.buffer)
//     .entry(2, self.weft)
//     .hub()?;

// gpu_.compute()
//     .root(&rig.stem)
//     .root(&form.stem)
//     .pipe(program.pipe.clone())
//     .bind(0, bind)
//     .dispatch(self.wheel.count)
//     .hub()
