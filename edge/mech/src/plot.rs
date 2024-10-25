pub use grid::*;

use super::*;

mod grid;

#[derive(Clone, Debug)]
pub struct Plot<'a> {
    pub mech: &'a Mech,
    pub gpu: &'a Gpu,
    pub shape: &'a Shape,
}

impl Plot<'_> {
    fn grid(&self, count: Hub<u32>) -> graph::Result<Hedge> {
        if let Rule::Nurbs(order) = self.shape.rule {
            self.grid_nurbs(order, count)
        } else {
            Err(anyhow!("Only Rule::Nurbs implemented"))?
        }
    }
    fn grid_nurbs(&self, order: u32, count: Hub<u32>) -> graph::Result<Hedge> {
        let rig = self
            .gpu
            .uniform()
            .field(order)
            .field(count.clone())
            .make()?;
        let buffer = self
            .gpu
            .blank(self.shape.span.buffer.clone())
            .mul(count.clone())
            .mul(2)
            .div(3)
            .hub()?;
        let bind = self
            .gpu
            .bind()
            .layout(self.mech.bin.basis.nurbs.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, self.shape.span.buffer.clone())
            .entry(2, buffer.clone())
            .hub()?;
        let root = self
            .gpu
            .command()
            .root(rig.root)
            .root(self.shape.span.root.clone())
            .compute(self.mech.bin.basis.nurbs.pipe.clone())
            .bind(bind)
            .dispatch(count.clone())
            .hub()?;
        self.shape.control.grid_basis(GridBasis {
            shape: self.shape,
            order,
            count,
            basis: Hedge { buffer, root },
        })
    }
}
