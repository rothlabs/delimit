use super::*;

#[derive(Clone, Debug)]
pub enum Control {
    Shape(Vec<Shape>),
    Hedge(Hedge),
}

pub struct GridBasis<'a> {
    pub shape: &'a Shape,
    pub order: u32,
    pub count: Hub<u32>,
    pub basis: Hedge,
}

impl Control {
    pub fn grid_basis(&self, grid: GridBasis) -> graph::Result<Hedge> {
        match &self {
            Control::Shape(shapes) => {
                let shape = shapes.first().unwrap();
                let _ = shape.grid(grid.count)?;
                Err(anyhow!("Control::Shape not implemented"))?
            }
            Control::Hedge(control) => self.grid_basis_hedge(grid, control),
        }
    }
    fn grid_basis_hedge(&self, grid: GridBasis, hedge: &Hedge) -> graph::Result<Hedge> {
        let stride = 1;
        let rig = grid
            .shape
            .gpu
            .uniform()
            .field(grid.order)
            .field(grid.count.clone())
            .field(stride)
            .field(grid.shape.dimension)
            .make()?;
        let buffer = grid
            .shape
            .gpu
            .blank(grid.basis.buffer.clone())
            .mul(grid.shape.dimension)
            .mul(stride)
            .div(grid.order)
            .hub()?;
        let bind = grid
            .shape
            .gpu
            .bind()
            .layout(grid.shape.mech.grid.basis.control.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, grid.basis.buffer)
            .entry(2, grid.shape.index.buffer.clone())
            .entry(3, hedge.buffer.clone())
            .entry(4, buffer.clone())
            .hub()?;
        let root = grid
            .shape
            .gpu
            .command()
            .root(grid.basis.root)
            .root(rig.root)
            .root(grid.shape.index.root.clone())
            .root(hedge.root.clone())
            .compute(grid.shape.mech.grid.basis.control.pipe.clone())
            .bind(bind)
            .dispatch(grid.count)
            .hub()?;
        Ok(Hedge { buffer, root })
    }
}
