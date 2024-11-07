use super::*;

mod grid;

pub struct Grid<'a> {
    pub chart: &'a Chart<'a>,
    pub counts: &'a [Hub<u32>],
}

impl<'a> Grid<'a> {
    pub fn hedge(&self) -> graph::Result<Hedge> {
        let wefts = self.wefts()?;
        let last_count = self.counts.last().ok_or(anyhow!("no counts"))?;
        let last_weft = wefts.last().ok_or(anyhow!("no wefts"))?;
        let mut area = 1.into();
        let mut warp = self.chart.shape.warp.clone();
        for rank in 0..self.chart.shape.flows.len() {
            let loom = grid::Loom {
                grid: self,
                rank,
                weft: wefts.get(rank).unwrap_or(last_weft),
                area: &area,
                count: self.counts.get(rank).unwrap_or(last_count),
            };
            warp = loom.hedge(&warp)?;
            area = loom.area.calc().mul(loom.count).hub()?;
        }
        Ok(warp)
    }
    fn wefts(&self) -> graph::Result<Vec<Weft>> {
        let mut wefts = vec![];
        for count in self.counts {
            let wheel = grid::Wheel {
                chart: self.chart,
                count,
            };
            wefts.push(wheel.weft()?);
        }
        Ok(wefts)
    }
}

#[derive(Default)]
pub struct Weft {
    pub travel: Option<Hedge>,
    pub orient: Option<Hedge>,
    pub spline: Vec<Option<Hedge>>,
}

impl Weft {
    fn travel(&self) -> graph::Result<&Hedge> {
        Ok(self.travel.as_ref().ok_or(anyhow!("no travel"))?)
    }
    fn orient(&self) -> graph::Result<&Hedge> {
        Ok(self.orient.as_ref().ok_or(anyhow!("no orient"))?)
    }
    fn spline(&self, order: usize) -> graph::Result<&Hedge> {
        let weft = self.spline.get(order).ok_or(anyhow!("no spline"))?;
        Ok(weft.as_ref().ok_or(anyhow!("no spline"))?)
    }
}

// fn loom(&self) -> grid::Loom {
//     grid::Loom {
//         grid: self,
//         rank: 0,
//         area: 1.into(),
//         weft: Weft::default(),
//         count: 0.into(),
//     }
// }

// let mut area: Hub<u32> = 1.into();
//         let mut warp = self.chart.shape.warp.clone();
//         for rank in 0..self.chart.shape.flows.len() {
//             let loom = grid::Loom {
//                 grid: self,
//                 rank,
//                 weft: wefts.get(rank).unwrap_or(last_weft),
//                 area: &area,
//                 count: self.counts.get(rank).unwrap_or(last_count),
//             };
//             warp = loom.hedge(&warp)?;
//             area = loom.area.calc().mul(loom.count).hub()?;
//         }
//         Ok(warp)

// let mut warp = self.chart.shape.warp.clone();
//         let mut loom = self.loom();
//         // let mut loom = grid::Loom {
//         //     grid: self,
//         //     rank: 0,
//         //     area: 1.into(),
//         //     weft: last_weft,
//         //     count: last_count,
//         // };
//         for rank in 0..self.chart.shape.flows.len() {
//             loom.rank = rank;
//             loom.weft = wefts.get(rank).cloned().unwrap_or(last_weft);
//             loom.count = self.counts.get(rank).unwrap_or(last_count);
//             warp = loom.hedge(&warp)?;
//             loom.area = loom.area.calc().mul(loom.count).hub()?;
//         }
//         Ok(warp)

// pub fn hedge(&self) -> graph::Result<Hedge> {
//     let wefts = self.wefts()?;
//     let last_count = self.counts.last().ok_or(anyhow!("no counts"))?;
//     let last_weft = wefts.last().ok_or(anyhow!("no wefts"))?;
//     let mut areas: Vec<Hub<u32>> = vec![1.into()];
//     let mut warps = vec![self.chart.shape.warp.clone()];
//     for rank in 0..self.chart.shape.flows.len() {
//         let loom = grid::Loom {
//             grid: self,
//             rank,
//             weft: wefts.get(rank).unwrap_or(last_weft),
//             area: areas.last().ok_or(anyhow!("no areas"))?,
//             count: self.counts.get(rank).unwrap_or(last_count),
//         };
//         let warp = warps.last().ok_or(anyhow!("no warps"))?;
//         warps.push(loom.hedge(warp)?);
//         areas.push(loom.area.calc().mul(loom.count).hub()?);
//     }
//     let plot = warps.last().cloned();
//     Ok(plot.ok_or(anyhow!("no warps"))?)
// }
