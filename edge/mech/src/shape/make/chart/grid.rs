use super::*;

mod loom;
mod wheel;

pub struct Wheel<'a> {
    pub plot: &'a Chart<'a>,
    pub count: &'a Hub<u32>,
}

impl<'a> Wheel<'a> {
    pub fn weft(&self) -> graph::Result<Weft> {
        let mut weft = Weft::default();
        let gpu = &self.plot.core.gpu;
        // let extrude_size = self.extrude_size()?;
        // let matrix_blank = gpu.blank(extrude_size).hub()?;
        for (order, form) in self.plot.shape.form.vector.iter().enumerate() {
            if let Some(form) = form {
                let mut root = JoinBuilder::default();
                let nurbs_size = self.nurbs_size(form)?;
                let label = format!("spin nurbs, order {}", order);
                let buffer = gpu.blank(nurbs_size).label(label).hub()?;
                let spin = self.spin(&buffer);
                if let Some(form) = &form.nurbs {
                    let rig = self.vector_rig(order, 0.into())?;
                    root.field(spin.nurbs(&rig, form)?);
                }
                let root = root.hub()?;
                weft.vector.push(Some(Hedge { buffer, root }));
            } else {
                weft.vector.push(None);
            }
        }
        Ok(weft)
    }
    fn spin(&self, buffer: &'a Hub<Grc<Buffer>>) -> wheel::Spin {
        wheel::Spin {
            charter: self,
            weft: buffer,
        }
    }
    // fn extrude_size(&self) -> graph::Result<Hub<u32>> {
    //     let gpu = &self.plot.core.gpu;
    //     let shape = &self.plot.shape;
    //     let size = if let Some(extrude) = &shape.weft.matrix.extrude {
    //         gpu.size(extrude.buffer.clone())
    //             .add(shape.dimension.pow(2))
    //             .mul(self.count.clone())
    //             .hub()?
    //     } else {
    //         0.into()
    //     };
    //     Ok(size)
    // }
    fn nurbs_size(&self, form: &form::Vector) -> graph::Result<Hub<u32>> {
        let gpu = &self.plot.core.gpu;
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
    fn vector_rig(&self, order: usize, offset: Hub<u32>) -> graph::Result<Hedge> {
        let uniform = self.plot.core.gpu.uniform();
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
            let warp = warps.last().ok_or(anyhow!("no plot"))?;
            warps.push(self.weave(rank).hedge(warp)?);
        }
        let plot = warps.last().cloned();
        Ok(plot.ok_or(anyhow!("no plot"))?)
    }
    fn weave(&self, rank: usize) -> loom::Weave {
        loom::Weave { loom: self, rank }
    }
}
