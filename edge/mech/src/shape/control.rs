use super::*;

#[derive(Clone, Debug)]
pub enum Control {
    Shape(Vec<Shape>),
    Hedge(Hedge),
}

// #[derive(Setters)]
pub struct GridBasis {
    pub gpu: Gpu,
    pub mech: Mech,
    pub order: u32,
    pub count: Hub<u32>,
    pub basis: Hedge,
    pub index: Hedge,
    // pub control: Control,
    pub dimension: u32,
}

impl Control {
    pub fn grid_basis(&self, grid: GridBasis) -> graph::Result<Hedge> {
        match &self {
            Control::Shape(shapes) => {
                let shape = shapes.first().unwrap();
                let _ = shape.grid(grid.count)?;
                Err(anyhow!("Control::Shape not implemented"))?
            }
            Control::Hedge(control) => {
                let stride = 1;
                let rig = grid
                    .gpu
                    .uniform()
                    .field(grid.order)
                    .field(grid.count.clone())
                    .field(stride)
                    .field(grid.dimension)
                    .make()?;
                let buffer = grid
                    .gpu
                    .blank(grid.basis.buffer.clone())
                    .mul(grid.dimension)
                    .mul(stride)
                    .div(grid.order)
                    .hub()?;
                let bind = grid
                    .gpu
                    .binder()
                    .layout(grid.mech.grid.basis.control.layout.clone())
                    .entry(0, rig.buffer)
                    .entry(1, grid.basis.buffer)
                    .entry(2, grid.index.buffer.clone())
                    .entry(3, control.buffer.clone())
                    .entry(4, buffer.clone())
                    .hub()?;
                let root = grid
                    .gpu
                    .command()
                    .root(grid.basis.root)
                    .root(rig.root)
                    .root(grid.index.root.clone())
                    .root(control.root.clone())
                    .compute(grid.mech.grid.basis.control.pipe.clone())
                    .bind(bind)
                    .dispatch(grid.count)
                    .hub()?;
                Ok(Hedge { buffer, root })
            }
        }
    }
}

// impl GridBasisControlBuilder {
    
// }