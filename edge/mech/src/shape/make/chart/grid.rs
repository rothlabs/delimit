use super::*;

mod loom;
mod wheel;

pub struct Wheel<'a> {
    pub chart: &'a Chart<'a>,
    pub count: &'a Hub<u32>,
}

impl<'a> Wheel<'a> {
    pub fn weft(&self) -> graph::Result<Weft> {
        let mut weft = Weft::default();
        let gpu = &self.chart.core.gpu;
        if let Some(form) = &self.chart.shape.form.linear {
            let mut root = JoinBuilder::default();
            let extrude_size = self.extrude_size(form)?;
            let buffer = gpu.blank(extrude_size).label("extrude").hub()?;
            let spin = self.spin(&buffer);
            let rig = self.rig(self.chart.shape.dimension as usize, 0.into())?;
            if let Some(form) = &form.extrude {
                root.field(spin.extrude(&rig, form)?);
            }
            let root = root.hub()?;
            weft.linear = Some(Hedge { buffer, root });
        }
        for (order, form) in self.chart.shape.form.spline.iter().enumerate() {
            if let Some(form) = form {
                let mut root = JoinBuilder::default();
                let spline_size = self.basis_size(form)?;
                let nurbs_size = self.nurbs_size(form)?;
                let size = spline_size.calc().add(nurbs_size).hub()?;
                let label = format!("nurbs {order}");
                let buffer = gpu.blank(size).label(label).hub()?;
                let spin = self.spin(&buffer);
                if let Some(form) = &form.basis {
                    let rig = self.rig(order, 0.into())?;
                    root.field(spin.basis(&rig, form)?);
                }
                if let Some(form) = &form.nurbs {
                    let rig = self.rig(order, spline_size)?;
                    root.field(spin.nurbs(&rig, form)?);
                }
                let root = root.hub()?;
                weft.spline.push(Some(Hedge { buffer, root }));
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
    fn extrude_size(&self, form: &form::Linear) -> graph::Result<Hub<u32>> {
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
            gpu.size(nurbs.buffer.clone())
                .mul(self.count.clone())
                .mul(2)
                .div(3)
                .hub()?
        } else {
            0.into()
        })
    }
    fn rig(&self, order: usize, offset: Hub<u32>) -> graph::Result<Hedge> {
        let uniform = self.chart.core.gpu.uniform();
        uniform
            .field(order as u32)
            .field(self.count.clone())
            .field(offset)
            .make()
    }
}

pub struct Loom<'a> {
    pub grid: &'a Grid<'a>,
    // per rank
    pub wefts: Vec<Weft>,
    // per rank
    pub areas: Vec<Hub<u32>>,
}

impl<'a> Loom<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let shape = &self.grid.chart.shape;
        let mut warps = vec![shape.warp.clone()];
        for rank in 0..shape.flows.len() {
            let warp = warps.last().ok_or(anyhow!("no warps"))?;
            warps.push(self.weave(rank).hedge(warp)?);
        }
        let plot = warps.last().cloned();
        Ok(plot.ok_or(anyhow!("no warps"))?)
    }
    fn weave(&self, rank: usize) -> loom::Weave {
        loom::Weave { loom: self, rank }
    }
}
