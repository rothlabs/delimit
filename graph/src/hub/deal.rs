use super::*;

impl Hub<String> {
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self.into())
    }
}

pub trait DealItem {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()>;
}

impl DealItem for Vec<Hub<String>> {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.vec(key, self.into())
    }
}

impl DealItem for Option<Hub<String>> {
    fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        if let Some(x) = self {
            deal.one(key, x.into())
        } else {
            Ok(())
        }
    }
}
