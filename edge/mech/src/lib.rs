use graph::*;

pub struct Vector {
    unit: Vec<Hub<f64>>
}

impl Solve for Vector {
    type Base = ();
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.deal("unit", deal)
    }
}