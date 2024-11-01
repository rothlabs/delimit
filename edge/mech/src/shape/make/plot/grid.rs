use super::*;

mod charter;
mod control;

pub struct Charter<'a> {
    pub plot: &'a Plot<'a>,
    pub count: &'a Hub<u32>,
}

impl<'a> Charter<'a> {
    pub fn span(&self) -> graph::Result<Span> {
        let mut span = Span::default();
        let gpu = &self.plot.core.gpu;
        // let extrude_size = self.extrude_size()?;
        // let matrix_blank = gpu.blank(extrude_size).hub()?;
        for (order, vector) in self.plot.shape.arch.vector.iter().enumerate() {
            if let Some(vector_span) = vector {
                let mut root = JoinBuilder::default();
                let nurbs_size = self.nurbs_size(vector_span)?;
                let buffer = gpu.blank(nurbs_size).hub()?;
                let part = self.part(&buffer);
                if let Some(span_hedge) = &vector_span.nurbs {
                    let rig = self.vector_rig(order, 0.into())?;
                    root.field(part.nurbs(&rig, span_hedge)?);
                }
                let root = root.hub()?;
                span.vector.push(Some(Hedge { buffer, root }));
            } else {
                span.vector.push(None);
            }
        }
        Ok(span)
        // self.control(&basis)
    }
    fn part(&self, buffer: &'a Hub<Grc<Buffer>>) -> charter::Part {
        charter::Part { rig: self, buffer }
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
    fn nurbs_size(&self, span: &arch::Vector) -> graph::Result<Hub<u32>> {
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
    fn vector_rig(&self, order: usize, offset: Hub<u32>) -> graph::Result<Hedge> {
        let uniform = self.plot.core.gpu.uniform();
        uniform
            .field(order as u32)
            .field(self.count.clone())
            .field(offset)
            .make()
    }
}


pub struct Control<'a> {
    pub grid: &'a Grid<'a>,
    // per rank
    pub spans: Vec<Span>,
    // per rank
    pub strides: Vec<Hub<u32>>,
}

impl<'a> Control<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let shape = &self.grid.plot.shape;
        let mut plots = vec![shape.points.clone()];
        for rank in 0..shape.jambs.len() {
            let plot = plots.last().ok_or(anyhow!("no plot"))?;
            plots.push(self.stage(rank, plot).hedge()?);
        }
        plots.last().cloned().ok_or(Err(anyhow!("no plot"))?)
    }
    fn stage(&self, rank: usize, plot: &'a Hedge) -> control::Stage{
        control::Stage {
            control: self,
            rank,
            plot,
        }
    }
}


// fn step(&self, i: usize, plot: &'a Hedge) -> graph::Result<Step> {
//     let last_basis = self.basis.last().ok_or(anyhow!("no basis"))?;
//     let index = &self.grid.plot.shape.index;
//     Ok(Step {
//         grid: &self.grid,
//         plot,
//         basis: self.basis.get(i).unwrap_or(last_basis),
//         index: index.get(i).ok_or(anyhow!("no index"))?,
//     })
// }

// if counts.len() > 1 {
//     let mut stride = counts[0].calc();
//     for count in counts.iter().skip(1) {
//         stride = stride.mul(count);
//     }
//     stride.hub()
// } else if counts.len() > 0 {
//     Ok(counts[0].clone())
// } else {
//     Ok(1.into())
// }
