use super::*;

mod basis;

pub struct Basis<'a> {
    pub plot: &'a Plot<'a>,
    pub vector: Vec<Option<Hedge>>,
    pub matrix: Option<Hedge>,
}

impl<'a> Basis<'a> {
    fn grid(&self, count: &'a Hub<u32>) -> basis::Grid {
        basis::Grid {
            basis: self,
            count,
        }
    }
}

pub struct Grid<'a> {
    pub plot: &'a Plot<'a>,
    pub count: &'a Hub<u32>,
}

impl<'a> Grid<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let mut basis = self.plot.basis();
        let gpu = &self.plot.core.gpu;
        // let extrude_size = self.extrude_size()?;
        // let matrix_blank = gpu.blank(extrude_size).hub()?;
        for (order, span) in self.plot.shape.span.vector.iter().enumerate() {
            let order = order as u32;
            if let Some(span) = span {
                let mut root = JoinBuilder::default();
                let nurbs_size = self.nurbs_size(span)?;
                let buffer = gpu.blank(nurbs_size).hub()?;
                let form = self.form(&buffer);
                if let Some(span) = &span.nurbs {
                    let rig = self.rig(order, 0.into())?;
                    root.field(form.nurbs(&rig, span)?);
                }
                let root = root.hub()?;
                basis.vector.push(Some(Hedge { buffer, root }));
            } else {
                basis.vector.push(None);
            }
        }
        Err(anyhow!("shape plot grid failed"))?
    }
    fn form(&self, buffer: &'a Hub<Grc<Buffer>>) -> Form {
        Form { grid: self, buffer }
    }
    // fn extrude_size(&self) -> graph::Result<Hub<u32>> {
    //     let gpu = &self.plot.core.gpu;
    //     let shape = &self.plot.shape;
    //     let size = if let Some(extrude) = &shape.span.matrix.extrude {
    //         gpu.size(extrude.buffer.clone())
    //             .add(shape.dimension.pow(2))
    //             .mul(self.count.clone())
    //             .hub()?
    //     } else {
    //         0.into()
    //     };
    //     Ok(size)
    // }
    fn nurbs_size(&self, span: &span::Vector) -> graph::Result<Hub<u32>> {
        let gpu = &self.plot.core.gpu;
        Ok(if let Some(nurbs) = &span.nurbs {
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
    fn rig(&self, order: u32, offset: Hub<u32>) -> graph::Result<Hedge> {
        let uniform = self.plot.core.gpu.uniform();
        uniform
            .field(order)
            .field(self.count.clone())
            .field(offset)
            .make()
    }
}

struct Form<'a> {
    grid: &'a Grid<'a>,
    buffer: &'a Hub<Grc<Buffer>>,
}

impl Form<'_> {
    fn nurbs(&self, rig: &Hedge, span: &Hedge) -> graph::Result<Hub<Mutation>> {
        let gpu = &self.grid.plot.core.gpu;
        let nurbs = &self.grid.plot.core.bank.plot.grid.basis.nurbs;
        let bind = gpu
            .bind()
            .layout(nurbs.layout.clone())
            .entry(0, rig.buffer.clone())
            .entry(1, span.buffer.clone())
            .entry(2, self.buffer.clone())
            .hub()?;
        gpu.command()
            .root(rig.root.clone())
            .root(span.root.clone())
            .compute(nurbs.pipe.clone())
            .bind(0, bind)
            .dispatch(self.grid.count.clone())
            .hub()
    }
}


