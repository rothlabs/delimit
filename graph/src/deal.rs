use super::*;

/// Trade a hub for another.
/// The implmentation should return the same semantic hub with different graph qualities.
pub trait Deal: Debug {
    // Did the deal read the unit?
    fn read(&self) -> bool {
        false
    }
    /// Did the deal mutate the unit?
    fn wrote(&self) -> bool {
        false
    }
    /// Set back of deal.
    fn back(&mut self, _: &Back) {}
    /// Handle one hub.
    fn one<'a>(&mut self, _: &str, _: View<'a>) -> Result<()> {
        Ok(())
    }
    /// Handle vector of hubes.
    fn vec<'a>(&mut self, _: &str, _: View<'a>) -> Result<()> {
        Ok(())
    }
    /// Handle map of hubes.
    fn map(&mut self, _: &mut Map) -> Result<()> {
        Ok(())
    }
}

