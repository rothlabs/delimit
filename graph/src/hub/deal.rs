use super::*;

impl Hub<String> {
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self.into())
    }
}

pub trait DealVec {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl DealVec for Vec<Hub<String>> {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, self.into())
    }
}