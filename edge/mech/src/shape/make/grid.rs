use super::*;

pub struct GridPlot<'a> {
    pub core: &'a Core,
    pub shape: &'a Shape,
    pub count: Hub<u32>,
}

impl GridPlot<'_> {
    pub fn grid(&self) -> graph::Result<Hedge> {
        let extrude_size = selfextrude_size
        let blank = self.core.gpu.blank(extrude_size).hub()?;

        for (order, span) in self.shape.span.vector.iter().enumerate() {
            let nurbs_size = if let Some(nurbs) = &span.nurbs {
                // When acceleration is included, remove mul(2).div(3) because plot row will be same length as nurbs row
                self.core
                    .gpu
                    .size(nurbs.buffer.clone())
                    .mul(count.clone())
                    .mul(2)
                    .div(3)
                    .hub()?
            } else {
                0.into()
            };
            let blank = self.core.gpu.blank(nurbs_size).hub()?;
            if let Some(nurbs) = &span.nurbs {}
        }
        Err(anyhow!("shape plot grid failed"))?
    }
    fn extrude_size(&self, count: Hub<u32>) -> graph::Result<Hub<u32>> {
        Ok(if let Some(extrude) = &self.shape.span.matrix.extrude {
            self.core
                .gpu
                .size(extrude.buffer.clone())
                .add(self.shape.dimension.pow(2))
                .mul(count.clone())
                .hub()?
        } else {
            0.into()
        })
    }
}