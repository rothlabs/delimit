use super::*;

impl Hub<String> {
    pub fn deal(&mut self, key: &str, deal: &mut dyn Deal) -> Result<()> {
        deal.one(key, self.into())
    }
}