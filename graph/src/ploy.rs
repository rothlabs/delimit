use async_trait::async_trait;

use super::*;

/// `Link` to domain-specific unit.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<dyn Engage<Base = T>>;

#[cfg(not(feature = "oneThread"))]
pub type PloyEdge<T> = Arc<RwLock<dyn Engage<Base = T>>>;
#[cfg(feature = "oneThread")]
pub type PloyEdge<T> = Rc<RefCell<dyn Engage<Base = T>>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Based + Adapt + Update + SetRoot + Debug {}

impl<E> Engage for E where E: Based + Adapt + Update + SetRoot + Debug {}

pub trait ToPloy {
    type Base;
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyEdge<Self::Base>;
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait Based {
    type Base: Payload;
    fn backed(&self, back: &Back) -> PloyEdge<Self::Base>;
    async fn solve(&self) -> Result<Hub<Self::Base>>;
    fn reckon(&self, task: Task) -> Result<Gain>;
}
