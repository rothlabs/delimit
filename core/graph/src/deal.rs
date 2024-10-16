use super::*;

/// Trade a hub for another.
/// The implmentation should return the same semantic hub with different graph qualities.
pub trait Deal: Debug + SendSync {
    /// Set back of deal.
    fn back(&mut self, _: &Back) {}
    /// Handle one hub.
    fn one(&mut self, _: &str, _: View) -> Result<()> {
        Ok(())
    }
    /// Handle vector of hubes.
    fn vec(&mut self, _: &str, _: ViewVec) -> Result<()> {
        Ok(())
    }
    /// Handle map of hubes.
    fn map(&mut self, _: &mut Map) -> Result<()> {
        Ok(())
    }
}
