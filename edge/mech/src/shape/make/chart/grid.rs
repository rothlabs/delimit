use super::*;

mod loom;
mod wheel;

pub struct Wheel<'a> {
    pub chart: &'a Chart<'a>,
    pub count: &'a Hub<u32>,
    pub size: Hub<u32>,
}

impl<'a> Wheel<'a> {
    pub fn weft(&self) -> Result<Weft> {
        let mut weft = Weft::default();
        self.travel(&mut weft)?;
        self.orient(&mut weft)?;
        self.spline(weft)
    }
    fn travel(&self, weft: &mut Weft) -> Result<()> {
        if let Some(form) = &self.chart.shape.form.travel {
            let mut stems = vec![];
            let extrude_size = self.extrude_size(form)?;
            let gpu = &self.chart.core.gpu;
            let buffer = gpu.blank(extrude_size).label("extrude").hub()?;
            let spin = self.spin(&buffer);
            if let Some(form) = &form.extrude {
                let rig = self.rig(self.chart.shape.dimension as usize, 0.into())?;
                stems.push(spin.extrude(&rig, form)?);
            }
            weft.travel = Some(Hedge { buffer, stems });
        }
        Ok(())
    }
    fn orient(&self, weft: &mut Weft) -> Result<()> {
        if let Some(form) = &self.chart.shape.form.orient {
            let mut stems = vec![];
            let revolve_size = self.revolve_size(form)?;
            let gpu = &self.chart.core.gpu;
            let buffer = gpu.blank(revolve_size).label("revolve").hub()?;
            let spin = self.spin(&buffer);
            if let Some(form) = &form.revolve {
                let rig = self.rig(self.chart.shape.dimension as usize, 0.into())?;
                stems.push(spin.revolve(&rig, form)?);
            }
            weft.orient = Some(Hedge { buffer, stems });
        }
        Ok(())
    }
    fn spline(&self, mut weft: Weft) -> Result<Weft> {
        for (order, form) in self.chart.shape.form.splines.iter().enumerate() {
            if let Some(form) = form {
                let mut stems = vec![];
                let spline_size = self.basis_size(form)?;
                let nurbs_size = self.nurbs_size(form)?;
                let size = spline_size.calc().add(nurbs_size).hub()?;
                let label = format!("nurbs {order}");
                let gpu = &self.chart.core.gpu;
                let buffer = gpu.blank(size).label(label).hub()?;
                let spin = self.spin(&buffer);
                if let Some(form) = &form.basis {
                    let rig = self.rig(order, 0.into())?;
                    stems.push(spin.basis(&rig, form)?);
                }
                if let Some(form) = &form.nurbs {
                    let rig = self.rig(order, spline_size)?;
                    stems.push(spin.nurbs(&rig, form)?);
                }
                println!("weft.spline.push: {}, order: {}", weft.spline.len(), order);
                weft.spline.push(Some(Hedge { buffer, stems }));
            } else {
                weft.spline.push(None);
            }
        }
        Ok(weft)
    }
    fn spin(&self, buffer: &'a Hub<Grc<Buffer>>) -> wheel::Spin {
        wheel::Spin {
            wheel: self,
            weft: buffer,
        }
    }
    fn extrude_size(&self, form: &form::Travel) -> graph::Result<Hub<u32>> {
        let gpu = &self.chart.core.gpu;
        Ok(if let Some(extrude) = &form.extrude {
            gpu.size(extrude.buffer.clone())
                .mul(self.count.clone())
                .mul(2)
                .hub()?
        } else {
            0.into()
        })
    }
    fn revolve_size(&self, form: &form::Orient) -> graph::Result<Hub<u32>> {
        let gpu = &self.chart.core.gpu;
        let dimension = self.chart.shape.dimension;
        Ok(if let Some(revolve) = &form.revolve {
            gpu.size(revolve.buffer.clone())
                .mul(self.count.clone())
                .mul(dimension * dimension * 2)
                .hub()?
        } else {
            0.into()
        })
    }
    fn basis_size(&self, form: &form::Spline) -> graph::Result<Hub<u32>> {
        let gpu = &self.chart.core.gpu;
        Ok(if let Some(basis) = &form.basis {
            gpu.size(basis.buffer.clone())
                .mul(self.count.clone())
                .hub()?
        } else {
            0.into()
        })
    }
    fn nurbs_size(&self, form: &form::Spline) -> graph::Result<Hub<u32>> {
        let gpu = &self.chart.core.gpu;
        Ok(if let Some(nurbs) = &form.nurbs {
            // When acceleration is included, remove mul(2).div(3) because plot row will be same length as nurbs row
            let size = gpu.size(nurbs.buffer.clone()).hub()?;
            size.calc().mul(&self.size).mul(2).mul(64).div(3).hub()?
            // gpu.size(nurbs.buffer.clone())
            //     .mul(self.count.clone())
            //     .div(3)
            //     .mul(2)
            //     // .add(63).div(64).mul(64)
            //     .hub()?
        } else {
            0.into()
        })
    }
    fn rig(&self, order: usize, offset: Hub<u32>) -> Result<Hedge> {
        let uniform = self.chart.core.gpu.uniform();
        Ok(uniform
            .field(order as u32)
            .field(self.count.clone())
            .field(offset)
            .make()?)
    }
}

pub struct Loom<'a> {
    pub grid: &'a Grid<'a>,
    pub rank: usize,
    pub weft: &'a Weft,
    pub area: &'a Hub<u32>,
    pub count: &'a Hub<u32>,
    pub size: Hub<u32>,
}

impl<'a> Loom<'a> {
    pub fn hedge(&self, warp: &Hedge) -> Result<Hedge> {
        let gpu = &self.grid.chart.core.gpu;
        let offsets = self.offsets()?;
        let size = offsets.last().ok_or(anyhow!("no offsets"))?;
        let label = format!("grid plot rank {}", self.rank);
        let buffer = gpu.blank(size).label(label).hub()?;
        let weave = self.weave(warp, &buffer)?;
        let mut stems = vec![];
        let flow = self.flow()?;
        let mut index = 0;

        // println!("loom size {order}");

        if let Some(flow) = &flow.travel {
            stems.push(weave.travel(loom::Trio {
                rig: self.rig(0, &0.into())?,
                weft: self.weft.travel()?,
                flow,
            })?);
            index += 1;
        }
        if let Some(flow) = &flow.orient {
            stems.push(weave.orient(loom::Trio {
                rig: self.rig(0, &0.into())?,
                weft: self.weft.orient()?,
                flow,
            })?);
            index += 1;
        }
        for (order, flow) in flow.splines.iter().enumerate() {
            if let Some(flow) = flow {
                let offset = offsets.get(index).ok_or(anyhow!("no offset"))?;
                // println!("weave spline {order}");
                stems.push(weave.spline(loom::Trio {
                    rig: self.rig(order, offset)?,
                    weft: self.weft.spline(order)?,
                    flow,
                })?);
                index += 1;
            }
        }
        Ok(Hedge { buffer, stems })
    }
    fn offsets(&self) -> Result<Vec<Hub<u32>>> {
        let chart = &self.grid.chart;
        let gpu = &chart.core.gpu;
        let flow = self.flow()?;
        let constant = chart.shape.dimension * (self.rank as u32 + 2) * 64;
        // println!("rank: {}, constant: {constant}", self.rank);
        // let expand = self.count.calc().mul(self.area).mul(constant).hub()?;
        let expand = self.size.calc().mul(constant).hub()?;
        let mut offsets = vec![0.into()];
        if let Some(flow) = &flow.travel {
            let size = gpu.size(&flow.buffer).div(2).mul(&expand).hub()?;
            // TODO: need to add last offset?
            offsets.push(size);
        }
        if let Some(flow) = &flow.orient {
            let size = gpu.size(&flow.buffer).div(2).mul(&expand).hub()?;
            // TODO: need to add last offset?
            offsets.push(size);
        }
        for (order, flow) in flow.splines.iter().enumerate() {
            if let Some(flow) = flow {
                // println!("push spline offset {order}");
                let size = gpu.size(&flow.buffer).div(order as u32 + 1).hub()?;
                let last = offsets.last().ok_or(anyhow!("no offsets"))?;
                offsets.push(size.calc().mul(&expand).add(last).hub()?);
            }
        }
        Ok(offsets)
    }
    fn flow(&self) -> Result<&Flow> {
        let flows = &self.grid.chart.shape.flows;
        Ok(flows.get(self.rank).ok_or(anyhow!("no flow"))?)
    }
    fn rig(&self, order: usize, offset: &Hub<u32>) -> Result<Hedge> {
        let dimension = self.grid.chart.shape.dimension;
        let uniform = self.grid.chart.core.gpu.uniform();
        Ok(uniform
            .field(self.rank as u32)
            .field(order as u32)
            .field(offset)
            .field(self.count)
            .field(self.area)
            .field(dimension)
            .make()?)
    }
    fn weave(&self, warp: &'a Hedge, plot: &'a Hub<Grc<Buffer>>) -> graph::Result<loom::Weave> {
        Ok(loom::Weave {
            loom: self,
            warp,
            plot,
            // count: self.count,
            // size: &self.size,
        })
    }
}
