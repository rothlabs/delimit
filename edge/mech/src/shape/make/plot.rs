use super::*;

mod grid;

#[derive(Default)]
pub struct Weft {
    pub matrix: Option<Hedge>,
    pub vector: Vec<Option<Hedge>>,
}

pub struct Grid<'a> {
    pub plot: &'a Plot<'a>,
    pub counts: &'a [Hub<u32>],
}

impl<'a> Grid<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let mut wefts = vec![];
        for count in self.counts {
            wefts.push(self.charter(count).weft()?);
        }
        let last_count = self.counts.last().ok_or(anyhow!("no counts"))?;
        let mut strides: Vec<Hub<u32>> = vec![1.into()];
        for i in 0..self.plot.shape.jambs.len() - 1 {
            let stride = strides.last().ok_or(anyhow!("no strides"))?.calc();
            let count = self.counts.get(i).unwrap_or(last_count);
            strides.push(stride.mul(count).hub()?);
        }
        self.control(wefts, strides)
    }
    fn charter(&self, count: &'a Hub<u32>) -> grid::Charter {
        grid::Charter {
            plot: self.plot,
            count,
        }
    }
    fn control(&self, wefts: Vec<Weft>, strides: Vec<Hub<u32>>) -> graph::Result<Hedge> {
        grid::Control {
            grid: self,
            wefts,
            strides,
        }
        .hedge()
    }
}
