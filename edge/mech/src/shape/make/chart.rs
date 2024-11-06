use super::*;

mod grid;

#[derive(Default)]
pub struct Weft {
    pub linear: Option<Hedge>,
    pub orient: Option<Hedge>,
    pub spline: Vec<Option<Hedge>>,
}

impl Weft {
    fn linear(&self) -> graph::Result<&Hedge> {
        Ok(self.linear.as_ref().ok_or(anyhow!("no linear"))?)
    }
    fn orient(&self) -> graph::Result<&Hedge> {
        Ok(self.orient.as_ref().ok_or(anyhow!("no orient"))?)
    }
    fn spline(&self, order: usize) -> graph::Result<&Hedge> {
        let weft = self.spline.get(order).ok_or(anyhow!("no spline"))?;
        Ok(weft.as_ref().ok_or(anyhow!("no spline"))?)
    }
}

pub struct Grid<'a> {
    pub chart: &'a Chart<'a>,
    pub counts: &'a [Hub<u32>],
}

impl<'a> Grid<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let mut wefts = vec![];
        for count in self.counts {
            wefts.push(self.wheel(count).weft()?);
        }
        let last_count = self.counts.last().ok_or(anyhow!("no counts"))?;
        let mut areas: Vec<Hub<u32>> = vec![1.into()];
        for i in 0..self.chart.shape.flows.len() - 1 {
            let area = areas.last().ok_or(anyhow!("no areas"))?.calc();
            let count = self.counts.get(i).unwrap_or(last_count);
            areas.push(area.mul(count).hub()?);
        }
        self.loom(wefts, areas).hedge()
    }
    fn wheel(&self, count: &'a Hub<u32>) -> grid::Wheel {
        grid::Wheel {
            chart: self.chart,
            count,
        }
    }
    fn loom(&self, wefts: Vec<Weft>, areas: Vec<Hub<u32>>) -> grid::Loom {
        // TODO: take chart, one weft, one area, and one count
        grid::Loom {
            grid: self,
            wefts,
            areas,
        }
    }
}
