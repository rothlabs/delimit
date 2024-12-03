use super::*;

pub struct Spin<'a> {
    pub wheel: &'a Wheel<'a>,
    // pub weft: &'a Hub<Grc<Buffer>>,
    pub size: Hub<u32>,
}

impl Spin<'_> {
    pub fn extrude(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.chart.core.bank.plot.grid.spin.extrude;
        self.weft(rig, form, program)
    }
    pub fn revolve(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let dimension = self.wheel.chart.shape.dimension;
        if dimension == 2 {
            let program = &self.wheel.chart.core.bank.plot.grid.spin.revolve2;
            return self.weft(rig, form, program);
        }
        Err(anyhow!(
            "only revolve 2D and 3D supported, found dimension {dimension}"
        ))?
    }
    pub fn basis(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.chart.core.bank.plot.grid.spin.basis;
        self.weft(rig, form, program)
    }
    pub fn nurbs(&self, rig: &Hedge, form: &Hedge) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let program = &self.wheel.chart.core.bank.plot.grid.spin.nurbs;
        self.weft(rig, form, program)
    }
    fn weft(
        &self,
        rig: &Hedge,
        form: &Hedge,
        program: &ComputeProgram,
    ) -> graph::Result<Hub<Grc<gpu::Action>>> {
        let gpu_ = &self.wheel.chart.core.gpu;
        let uniform = &gpu_.store.rig.group;
        let storage = &gpu_.store.storage.group;
        let stems = rig.stems.with(&form.stems);
        gpu::dispatch()
            .pipe(&program.pipe)
            .bind(gpu::bind().slot(0).group(storage).hub()?)
            .bind(
                gpu::bind()
                    .slot(1)
                    .group(uniform)
                    .offset(&rig.offset)
                    .hub()?,
            )
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
