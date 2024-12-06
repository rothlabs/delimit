use super::*;

mod loom;
mod wheel;

pub struct Wheel<'a> {
    // pub chart: &'a ChartBank<'a>,
    pub mech: &'a Mech,
    pub shape: &'a Shape,
    pub count: &'a Hub<u32>,
}

impl<'a> Wheel<'a> {
    pub fn weft(&self) -> Result<Weft> {
        let mut weft = Weft::default();
        self.travel(&mut weft)?;
        self.orient(&mut weft)?;
        self.spline(weft)
    }
    fn travel(&self, weft: &mut Weft) -> Result<()> {
        if let Some(form) = &self.shape.form.travel {
            let gpu = &self.mech.gpu;
            let mut stems = vec![];
            let size = self.extrude_size(form);
            let offset = gpu.store.topic(&size);
            // let buffer = gpu.blank(&size).label("extrude").hub()?;
            let spin = self.spin();
            if let Some(form) = &form.extrude {
                let rig = self.rig(self.shape.dimension, &form.index, &offset, &size)?;
                stems.push(spin.extrude(&rig, form)?);
            }
            weft.travel = Some(Hedge {
                index: offset,
                size,
                stems,
            });
        }
        Ok(())
    }
    fn orient(&self, weft: &mut Weft) -> Result<()> {
        if let Some(form) = &self.shape.form.orient {
            let gpu = &self.mech.gpu;
            let mut stems = vec![];
            let size = self.revolve_size(form);
            let offset = gpu.store.topic(&size);
            // let buffer = gpu.blank(&size).label("revolve").hub()?;
            let spin = self.spin();
            if let Some(form) = &form.revolve {
                let rig = self.rig(self.shape.dimension, &form.index, &offset, &size)?;
                stems.push(spin.revolve(&rig, form)?);
            }
            weft.orient = Some(Hedge {
                index: offset,
                size,
                stems,
            });
        }
        Ok(())
    }
    fn spline(&self, mut weft: Weft) -> Result<Weft> {
        for form in &self.shape.form.splines {
            // if let Some(form) = form {
            let order = form.order;
            let mut stems = vec![];
            let spline_size = self.basis_size(form);
            let nurbs_size = form.nurbs_size(self.count);
            let nurbs_expand = nurbs_size.math().div(3).mul(2).hub();
            let nurbs_length = nurbs_size.math().div(3).div(order).hub();
            let size = spline_size.math().add(&nurbs_expand).hub();
            // let label = format!("nurbs {order}");
            let gpu = &self.mech.gpu;
            let offset = gpu.store.topic(&size);
            // let buffer = gpu.blank(size).label(label).hub()?;
            let mut spin = self.spin(); // &buffer
            if let Some(form) = &form.basis {
                // spline_size should be devided by 3 and order to get number of invocations
                let rig = self.rig(order, &form.index, &offset, &spline_size)?;
                // TODO: set spin size before calling basis
                stems.push(spin.basis(&rig, form)?);
            }
            if let Some(form) = &form.nurbs {
                let offset = spline_size.math().add(&offset).hub();
                let rig = self.rig(order, &form.index, &offset, &nurbs_length)?;
                spin.size = nurbs_length.math().add(63).div(64).hub();
                stems.push(spin.nurbs(&rig, form)?);
            }
            weft.spline.insert(
                order,
                Hedge {
                    index: offset,
                    size,
                    stems,
                },
            );
            // } else {
            //     weft.spline.push(None);
            // }
        }
        Ok(weft)
    }
    fn spin(&self) -> wheel::Spin {
        // , buffer: &'a Hub<Grc<Buffer>>
        wheel::Spin {
            wheel: self,
            // weft: buffer,
            size: 0.into(),
        }
    }
    fn extrude_size(&self, form: &form::Travel) -> Hub<u32> {
        // let gpu = &self.chart.core.gpu;
        if let Some(extrude) = &form.extrude {
            //gpu.size(extrude.buffer.clone())
            extrude.size.math().mul(self.count.clone()).mul(2).hub()
        } else {
            0.into()
        }
    }
    fn revolve_size(&self, form: &form::Orient) -> Hub<u32> {
        // let gpu = &self.chart.core.gpu;
        let dimension = self.shape.dimension;
        if let Some(revolve) = &form.revolve {
            // gpu.size(revolve.buffer.clone())
            revolve
                .size
                .math()
                .mul(self.count.clone())
                .mul(dimension * dimension * 2)
                .hub()
        } else {
            0.into()
        }
    }
    fn basis_size(&self, form: &form::Spline) -> Hub<u32> {
        // let gpu = &self.chart.core.gpu;
        if let Some(basis) = &form.basis {
            // gpu.size(basis.buffer.clone())
            basis.size.math().mul(self.count.clone()).hub()
        } else {
            0.into()
        }
    }
    // fn nurbs_size(&self, form: &form::Spline) -> graph::Result<Hub<u32>> {
    //     let gpu = &self.chart.core.gpu;
    //     Ok(if let Some(nurbs) = &form.nurbs {
    //         // When acceleration is included, remove mul(2).div(3) because plot row will be same length as nurbs row
    //         // let size = gpu.size(nurbs.buffer.clone()).hub()?;
    //         // size.calc().mul(&self.size).mul(2).mul(64).div(3).hub()?
    //         gpu.size(nurbs.buffer.clone()).hub()?
    //             // .mul(self.count.clone())
    //             // .div(3)
    //             // .mul(2)
    //             // .hub()?
    //     } else {
    //         0.into()
    //     })
    // }
    fn rig(
        &self,
        order: u32,
        form_offset: &Hub<u32>,
        offset: &Hub<u32>,
        length: &Hub<u32>,
    ) -> Result<Hedge> {
        let gpu = &self.mech.gpu;
        let buffer = &gpu.store.rig.buffer;
        let vector = VectorBuilder::default()
            .field(order)
            .field(self.count.clone())
            .field(form_offset)
            .field(offset)
            .field(length)
            .hub()?;
        let offset = gpu.store.rig();
        let stem = gpu.writer(buffer).data(vector).index(&offset).hub()?;
        Ok(Hedge {
            index: offset,
            size: 64.into(),
            stems: vec![stem],
        })
    }
}

// let uniform = self.chart.core.gpu.uniform();
// Ok(uniform
//     .field(order as u32)
//     .field(self.count.clone())
//     .field(offset)
//     .field(length)
//     .make()?)
// let size = build.fields.len() as u64 * 4;

type OffsetsAndLengths = Result<(Vec<Hub<u32>>, Vec<Hub<u32>>)>;

pub struct Loom<'a> {
    pub grid: &'a Grid<'a>,
    pub rank: usize,
    pub weft: &'a Weft,
    pub area: &'a Hub<u32>,
    pub count: &'a Hub<u32>,
    // pub size: Hub<u32>,
}

impl<'a> Loom<'a> {
    pub fn hedge(&self, warp: &Hedge) -> Result<Hedge> {
        let gpu = &self.grid.mech.gpu;
        let (offsets, lengths) = self.offsets_and_lengths()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?.clone();
        let main_offset = gpu.store.topic(&size);
        // let label = format!("grid plot rank {}", self.rank);
        // let buffer = gpu.blank(size).label(label).hub()?;
        let mut weave = self.weave(warp)?;
        let mut stems = vec![];
        let flow = self.flow()?;
        let mut index = 0;
        if let Some(flow) = &flow.travel {
            let length = lengths.get(index).ok_or(anyhow!("no length"))?;
            let weft = self.weft.travel()?;
            stems.push(weave.travel(loom::Trio {
                ///////////////////////////////// added main offset
                rig: self.rig(
                    0,
                    &warp.index,
                    &weft.index,
                    &flow.index,
                    &main_offset,
                    length,
                )?,
                weft,
                flow,
            })?);
            index += 1;
        }
        if let Some(flow) = &flow.orient {
            ///////////////////////////////// added main offset
            let offset = offsets
                .get(index)
                .ok_or(anyhow!("no offset"))?
                .math()
                .add(&main_offset)
                .hub();
            let length = lengths.get(index).ok_or(anyhow!("no length"))?;
            let weft = self.weft.orient()?;
            stems.push(weave.orient(loom::Trio {
                rig: self.rig(0, &warp.index, &weft.index, &flow.index, &offset, length)?,
                weft,
                flow,
            })?);
            index += 1;
        }
        for flow in &flow.splines {
            let offset = offsets
                .get(index)
                .ok_or(anyhow!("no offset"))?
                .math()
                .add(&main_offset)
                .hub();
            let length = lengths.get(index).ok_or(anyhow!("no length"))?;
            weave.size = length.math().add(63).div(64).hub();
            let weft = self.weft.spline(flow.order)?;
            stems.push(weave.spline(loom::Trio {
                rig: self.rig(
                    flow.order,
                    &warp.index,
                    &weft.index,
                    &flow.hedge.index,
                    &offset,
                    length,
                )?,
                weft,
                flow: &flow.hedge,
            })?);
            index += 1;
        }
        Ok(Hedge {
            index: main_offset,
            size,
            stems,
        })
    }
    fn offsets_and_lengths(&self) -> OffsetsAndLengths {
        let chart = &self.grid;
        // let gpu = &chart.core.gpu;
        let flow = self.flow()?;
        let constant = chart.shape.dimension * (self.rank as u32 + 2);
        let expand = self.count.math().mul(self.area).mul(constant).hub();
        let mut offsets = vec![0.into()];
        let mut lengths = vec![];
        if let Some(flow) = &flow.travel {
            // let size = gpu.size(&flow.buffer).div(2).mul(&expand).hub()?;
            let size = flow.size.math().div(2).mul(&expand).hub();
            // TODO: need to add last offset?
            offsets.push(size);
        }
        if let Some(flow) = &flow.orient {
            // let size = gpu.size(&flow.buffer).div(2).mul(&expand).hub()?;
            let size = flow.size.math().div(2).mul(&expand).hub();
            // TODO: need to add last offset?
            offsets.push(size);
        }
        for flow in &flow.splines {
            // if let Some(flow) = flow {
            // let size = gpu.size(&flow.buffer).div(order as u32 + 1).hub()?;
            let size = flow.hedge.size.math().div(flow.order + 1).hub();
            let last = offsets.last().ok_or(anyhow!("no offsets"))?;
            offsets.push(size.math().mul(&expand).add(last).hub());
            lengths.push(size.math().mul(self.count).mul(self.area).hub());
            // }
        }
        Ok((offsets, lengths))
    }
    fn flow(&self) -> Result<&Flow> {
        let flows = &self.grid.shape.flows;
        Ok(flows.get(self.rank).ok_or(anyhow!("no flow"))?)
    }
    fn rig(
        &self,
        order: u32,
        warp: &Hub<u32>,
        weft: &Hub<u32>,
        flow: &Hub<u32>,
        offset: &Hub<u32>,
        length: &Hub<u32>,
    ) -> Result<Hedge> {
        let dimension = self.grid.shape.dimension;
        let gpu = &self.grid.mech.gpu;
        let buffer = &gpu.store.rig.buffer;
        let vector = VectorBuilder::default()
            .field(self.rank as u32)
            .field(order)
            .field(self.count)
            .field(self.area)
            .field(dimension)
            .field(warp)
            .field(weft)
            .field(flow)
            .field(offset)
            .field(length)
            .hub()?;
        let offset = gpu.store.rig();
        let stem = gpu.writer(buffer).data(vector).index(&offset).hub()?;
        Ok(Hedge {
            index: offset,
            size: 64.into(),
            stems: vec![stem],
        })
    }
    fn weave(
        &self,
        warp: &'a Hedge,
        // plot: &'a Hub<Grc<Buffer>>,
    ) -> graph::Result<loom::Weave> {
        Ok(loom::Weave {
            loom: self,
            warp,
            // plot,
            size: 0.into(),
        })
    }
}

// let uniform = self.grid.chart.core.gpu.uniform();
// Ok(uniform
//     .field(self.rank as u32)
//     .field(order as u32)
//     .field(offset)
//     .field(length)
//     .field(self.count)
//     .field(self.area)
//     .field(dimension)
//     .make()?)
