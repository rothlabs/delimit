use super::*;

pub struct Plot<'a> {
    pub core: &'a Core,
    pub shape: &'a Shape,
    pub count: Hub<u32>,
}

impl Plot<'_> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let extrude_size = self.extrude_size()?;
        let matrix_blank = self.core.gpu.blank(extrude_size).hub()?;
        for (order, span) in self.shape.span.vector.iter().enumerate() {
            let nurbs_size = self.nurbs_size(span)?;
            let blank = self.core.gpu.blank(nurbs_size).hub()?;
            
        }
        Err(anyhow!("shape plot grid failed"))?
    }
    fn extrude_size(&self) -> graph::Result<Hub<u32>> {
        Ok(if let Some(extrude) = &self.shape.span.matrix.extrude {
            self.core
                .gpu
                .size(extrude.buffer.clone())
                .add(self.shape.dimension.pow(2))
                .mul(self.count.clone())
                .hub()?
        } else {
            0.into()
        })
    }
    fn nurbs_size(&self, span: &span::Vector) -> graph::Result<Hub<u32>> {
        Ok(if let Some(nurbs) = &span.nurbs {
            // When acceleration is included, remove mul(2).div(3) because plot row will be same length as nurbs row
            self.core
                .gpu
                .size(nurbs.buffer.clone())
                .mul(self.count.clone())
                .mul(2)
                .div(3)
                .hub()?
        } else {
            0.into()
        })
    }
    fn rig(&self, order: u32, offset: Hub<u32>) -> graph::Result<Hedge> {
        self.core
            .gpu
            .uniform()
            .field(order)
            .field(self.count.clone())
            .field(offset.clone())
            .make()
    }
}

struct Form<'a> {
    core: &'a Core,
    shape: &'a Shape,
    count: &'a Hub<u32>,
    rig: &'a Hedge,
    offset: &'a Hub<u32>,
    span: &'a Hedge,
    out: &'a Hub<Grc<Buffer>>,
}

impl Form<'_> {
    fn nurbs(&self, order: u32) -> graph::Result<Hub<Mutation>> {
        let bind = self.core
            .gpu
            .bind()
            .layout(self.core.bank.plot.grid.basis.nurbs.layout.clone())
            .entry(0, self.rig.buffer.clone())
            .entry(1, self.span.buffer.clone())
            .entry(2, self.out.clone())
            .hub()?;
        self.core
            .gpu
            .command()
            .root(self.rig.root.clone())
            .root(self.span.root.clone())
            .compute(self.core.bank.plot.grid.basis.nurbs.pipe.clone())
            .bind(0, bind)
            .dispatch(self.count.clone())
            .hub()
    }
}


pub struct GridBasis<'a> {
    basis: Basis<'a>,
    count: Hub<u32>,
}