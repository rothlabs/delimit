use super::*;

mod chart;

pub struct ChartBank<'a> {
    pub mech: &'a Mech,
    pub shape: &'a Shape,
}

impl<'a> ChartBank<'a> {
    pub fn grid(&self, counts: &'a [Hub<u32>]) -> Result<Hedge> {
        chart::Grid {
            // chart: self,
            mech: self.mech,
            shape: self.shape,
            counts,
        }
        .hedge()
    }
}

// Ok(Plot {
//     hedge: chart::Grid { chart: self, counts }.hedge()?,
//     shape: self.shape.clone()
// })

// BasisBuilder {
//     plot: Some(self),
//     ..Default::default()
// }
// // let mut basis = BasisBuilder::default();
// // basis.plot(self);
// // basis

// impl Basis<'_> {
//     fn new
// }

// // TODO: make it a generic trait in graph crate
// fn last(sum: Option<&Hub<u32>>) -> Hub<u32> {
//     sum.cloned().unwrap_or(0.into())
// }

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

// let mut span_offsets = vec![];
// if let Some(nurbs) = &span.nurbs {
//     // When acceleration is included, remove mul(2).div(3) because plot row will be same length as nurbs row
//     let mut size = self.core.gpu.size(nurbs.buffer.clone()).mul(count.clone()).mul(2).div(3).hub()?;
//     let sum = Sum {
//         fields: vec![last(span_offsets.last()), size.hub()?],
//     };
//     span_offsets.push(sum.gate()?.into());
// }
// let span_size = last(span_offsets.last());
// let offset = Divide {
//     dividend: span_size.clone(),
//     divisor: (order as u64).into(),
// };
// let offset = Sum {
//     fields: vec![last(control_offsets.last()), offset.gate()?.into()],
// };
// control_offsets.push(offset.gate()?.into());
// let blank = self.core.gpu.blank(span_size).hub()?;
// for (i, span) in vector_spans.iter().enumerate() {
//     if let span::VectorRule::Nurbs = span.rule {}
// }
