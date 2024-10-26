use super::*;

pub struct Plot<'a> {
    pub bin: &'a Bin,
    pub gpu: &'a Gpu,
    pub shape: &'a Shape,
}

impl Plot<'_> {
    pub fn grid(&self, count: Hub<u32>) -> graph::Result<Hedge> {
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
            .layout(self.bin.plot.grid.basis.nurbs.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, self.shape.span.buffer.clone())
            .entry(2, buffer.clone())
            .hub()?;
        let root = self
            .gpu
            .command()
            .root(rig.root)
            .root(self.shape.span.root.clone())
            .compute(self.bin.plot.grid.basis.nurbs.pipe.clone())
            .bind(bind)
            .dispatch(count.clone())
            .hub()?;
        Basis {
            bin: self.bin,
            gpu: self.gpu,
            shape: self.shape,
            order,
            count,
            hedge: Hedge { buffer, root },
        }
        .control()
    }
}

pub struct Basis<'a> {
    pub bin: &'a Bin,
    pub gpu: &'a Gpu,
    pub shape: &'a Shape,
    pub order: u32,
    pub count: Hub<u32>,
    pub hedge: Hedge,
}

impl Basis<'_> {
    pub fn control(&self) -> graph::Result<Hedge> {
        match &self.shape.control {
            Control::Shape(_) => {
                // let shape = shapes.first().unwrap();
                // let _ = shape.grid(grid.count)?;
                Err(anyhow!("Control::Shape not implemented"))?
            }
            Control::Hedge(control) => self.control_hedge(control),
        }
    }
    fn control_hedge(&self, control: &Hedge) -> graph::Result<Hedge> {
        let stride = 1;
        let rig = self
            .gpu
            .uniform()
            .field(self.order)
            .field(self.count.clone())
            .field(stride)
            .field(self.shape.dimension)
            .make()?;
        let buffer = self
            .gpu
            .blank(self.hedge.buffer.clone())
            .mul(self.shape.dimension)
            .mul(stride)
            .div(self.order)
            .hub()?;
        let bind = self
            .gpu
            .bind()
            .layout(self.bin.plot.grid.basis.control.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, self.hedge.buffer.clone())
            .entry(2, self.shape.index.buffer.clone())
            .entry(3, control.buffer.clone())
            .entry(4, buffer.clone())
            .hub()?;
        let root = self
            .gpu
            .command()
            .root(rig.root)
            .root(self.hedge.root.clone())
            .root(self.shape.index.root.clone())
            .root(control.root.clone())
            .compute(self.bin.plot.grid.basis.control.pipe.clone())
            .bind(bind)
            .dispatch(self.count.clone())
            .hub()?;
        Ok(Hedge { buffer, root })
    }
}
