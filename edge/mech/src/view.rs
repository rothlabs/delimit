use super::*;
// use bank::*;

// mod bank;
mod make;

#[derive(Clone, Debug)]
pub struct View {
    pub mech: Mech,
    pub port: Viewport,
    // pub bank: Grc<Bank>,
}

impl View {
    pub fn new(mech: Mech, port: Viewport) -> Result<Self> {
        Ok(Self {
            mech,
            // bank: Bank::new(&port)?.into(),
            port,
        })
    }
    // TODO: also impl on shape to creating everything needed to render automatically
    pub fn plot(&self, plot: impl Into<Hub<Plot>>) -> make::Chart {
        make::Chart {
            view: self.clone(),
            plot: plot.into(),
        }
    }
}
