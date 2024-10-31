use super::*;

mod grid;

#[derive(Default)]
pub struct Basis {
    pub matrix: Option<Hedge>,
    pub vector: Vec<Option<Hedge>>,
}

pub struct Grid<'a> {
    pub plot: &'a Plot<'a>,
    pub counts: &'a [Hub<u32>],
}

impl<'a> Grid<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let mut basis = vec![];
        for count in self.counts {
            basis.push(self.array(count).basis()?);
        }
        let last_count = self.counts.last().ok_or(anyhow!("no counts"))?;
        let mut strides: Vec<Hub<u32>> = vec![1.into()];
        for i in 0..self.plot.shape.index.len() - 1 {
            let stride = strides.last().cloned().unwrap_or(1.into()).calc();
            let count = self.counts.get(i).cloned().unwrap_or(last_count.clone());
            strides.push(stride.mul(count).hub()?);
        }
        self.control(basis, strides)
    }
    fn array(&self, count: &'a Hub<u32>) -> Array {
        Array {
            plot: self.plot,
            count,
        }
    }
    fn control(&self, basis: Vec<Basis>, strides: Vec<Hub<u32>>) -> graph::Result<Hedge> {
        grid::Control {
            grid: self,
            basis,
            strides,
        }
        .hedge()
    }
}

pub struct Array<'a> {
    pub plot: &'a Plot<'a>,
    pub count: &'a Hub<u32>,
}

impl<'a> Array<'a> {
    pub fn basis(&self) -> graph::Result<Basis> {
        let mut basis = Basis::default();
        let gpu = &self.plot.core.gpu;
        // let extrude_size = self.extrude_size()?;
        // let matrix_blank = gpu.blank(extrude_size).hub()?;
        for (order, span) in self.plot.shape.span.vector.iter().enumerate() {
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
        Ok(basis)
        // self.control(&basis)
    }
    fn form(&self, buffer: &'a Hub<Grc<Buffer>>) -> grid::Form {
        grid::Form {
            array: self,
            buffer,
        }
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
    fn rig(&self, order: usize, offset: Hub<u32>) -> graph::Result<Hedge> {
        let uniform = self.plot.core.gpu.uniform();
        uniform
            .field(order as u32)
            .field(self.count.clone())
            .field(offset)
            .make()
    }
}
