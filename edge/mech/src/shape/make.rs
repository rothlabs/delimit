use super::*;

pub struct Plot<'a> {
    // pub bank: &'a Bank,
    // pub gpu: &'a Gpu,
    pub core: &'a Core,
    pub shape: &'a Shape,
}

fn last(sum: Option<&Hub<u64>>) -> Hub<u64> {
    sum.cloned().unwrap_or(0_u64.into())
}

pub struct Basis<'a> {
    pub core: &'a Core,
    pub shape: &'a Shape,
    pub vector: Vec<Hedge>,
    pub matrix: Hedge,
    pub count: Hub<u32>,
}

impl Plot<'_> {
    pub fn grid(&self, count: Hub<u32>) -> graph::Result<Hedge> {
        // TODO: make it a generic trait in graph crate
        let mut control_offsets: Vec<Hub<u64>> = vec![];
        for (order, vector_spans) in self.shape.span.vector.iter().enumerate() {
            let mut span_offsets = vec![];
            for span in vector_spans {
                let buffer = span.hedge.buffer.clone();
                let mut size = self.core.gpu.size(buffer).mul(count.clone());
                // When acceleration is included, remove this because plot row will be same length as nurbs row
                if let span::VectorRule::Nurbs = span.rule {
                    size = size.mul(2).div(3);
                }
                let sum = Sum {
                    fields: vec![last(span_offsets.last()), size.hub()?],
                };
                span_offsets.push(sum.gate()?.into());
            }
            let span_size = last(span_offsets.last());
            let offset = Divide {
                dividend: span_size.clone(),
                divisor: (order as u64).into(),
            };
            let offset = Sum {
                fields: vec![last(control_offsets.last()), offset.gate()?.into()],
            };
            control_offsets.push(offset.gate()?.into());
            let blank = self.core.gpu.blank(span_size).hub()?;
            for (i, span) in vector_spans.iter().enumerate() {
                if let span::VectorRule::Nurbs = span.rule {}
            }
        }
        Err(anyhow!("shape plot grid failed"))?
    }
    // fn grid_nurbs(&self, count: Hub<u32>, order: usize, buffer: Hub<Grc<Buffer>>, offset: Hub<u64>) -> graph::Result<Hedge> {
    //     let rig = self
    //         .gpu
    //         .uniform()
    //         .field(order as u32)
    //         .field(count.clone())
    //         .make()?;
    //     let bind = self
    //         .gpu
    //         .bind()
    //         .layout(self.bank.plot.grid.basis.nurbs.layout.clone())
    //         .entry(0, rig.buffer)
    //         .entry(1, self.shape.span.vector[order].hedge.buffer.clone())
    //         .entry(2, buffer.clone())
    //         .hub()?;
    //     let root = self
    //         .gpu
    //         .command()
    //         .root(rig.root)
    //         .root(self.shape.span.root.clone())
    //         .compute(self.bank.plot.grid.basis.nurbs.pipe.clone())
    //         .bind(0, bind)
    //         .dispatch(count.clone())
    //         .hub()?;
    //     Basis {
    //         bin: self.bank,
    //         gpu: self.gpu,
    //         shape: self.shape,
    //         order,
    //         count,
    //         hedge: Hedge { buffer, root },
    //     }
    //     .control()
    // }
}

// fn grid_nurbs(&self, order: u32, count: Hub<u32>) -> graph::Result<Hedge> {
//     let rig = self
//         .gpu
//         .uniform()
//         .field(order)
//         .field(count.clone())
//         .make()?;
//     let buffer = self
//         .gpu
//         .blank(self.shape.span.buffer.clone())
//         .mul(count.clone())
//         .mul(2)
//         .div(3)
//         .hub()?;
//     let bind = self
//         .gpu
//         .bind()
//         .layout(self.bank.plot.grid.basis.nurbs.layout.clone())
//         .entry(0, rig.buffer)
//         .entry(1, self.shape.span.buffer.clone())
//         .entry(2, buffer.clone())
//         .hub()?;
//     let root = self
//         .gpu
//         .command()
//         .root(rig.root)
//         .root(self.shape.span.root.clone())
//         .compute(self.bank.plot.grid.basis.nurbs.pipe.clone())
//         .bind(0, bind)
//         .dispatch(count.clone())
//         .hub()?;
//     Basis {
//         bin: self.bank,
//         gpu: self.gpu,
//         shape: self.shape,
//         order,
//         count,
//         hedge: Hedge { buffer, root },
//     }
//     .control()
// }

// pub struct Basis<'a> {
//     pub bin: &'a Bank,
//     pub gpu: &'a Gpu,
//     pub shape: &'a Shape,
//     pub order: u32,
//     pub count: Hub<u32>,
//     pub hedge: Hedge,
// }

// impl Basis<'_> {
//     pub fn control(&self) -> graph::Result<Hedge> {
//         match &self.shape.control {
//             Control::Shape(_) => {
//                 // let shape = shapes.first().unwrap();
//                 // let _ = shape.grid(grid.count)?;
//                 Err(anyhow!("Control::Shape not implemented"))?
//             }
//             Control::Hedge(control) => self.control_hedge(control),
//         }
//     }
//     fn control_hedge(&self, control: &Hedge) -> graph::Result<Hedge> {
//         let stride = 1;
//         let rig = self
//             .gpu
//             .uniform()
//             .field(self.order)
//             .field(self.count.clone())
//             .field(stride)
//             .field(self.shape.dimension)
//             .make()?;
//         let buffer = self
//             .gpu
//             .blank(self.hedge.buffer.clone())
//             .mul(self.shape.dimension)
//             .mul(stride)
//             .div(self.order)
//             .hub()?;
//         let bind = self
//             .gpu
//             .bind()
//             .layout(self.bin.plot.grid.basis.control.layout.clone())
//             .entry(0, rig.buffer)
//             .entry(1, self.hedge.buffer.clone())
//             .entry(2, self.shape.index.buffer.clone())
//             .entry(3, control.buffer.clone())
//             .entry(4, buffer.clone())
//             .hub()?;
//         let root = self
//             .gpu
//             .command()
//             .root(rig.root)
//             .root(self.hedge.root.clone())
//             .root(self.shape.index.root.clone())
//             .root(control.root.clone())
//             .compute(self.bin.plot.grid.basis.control.pipe.clone())
//             .bind(0, bind)
//             .dispatch(self.count.clone())
//             .hub()?;
//         Ok(Hedge { buffer, root })
//     }
// }
