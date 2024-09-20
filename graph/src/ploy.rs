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
pub trait Engage:
    Reckon + Based + AdaptGet + AdaptSet + AddRoot + Update + SetRoot + Debug
{
}

impl<E> Engage for E where
    E: Reckon + Based + AdaptGet + AdaptSet + AddRoot + Update + SetRoot + Debug
{
}

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
}

// #[cfg_attr(not(feature = "oneThread"), async_trait)]
// #[cfg_attr(feature = "oneThread", async_trait(?Send))]
// impl<T> Based for Box<dyn Engage<Base = T>>
// where
//     T: 'static + Payload,
// {
//     type Base = T;
//     fn backed(&self, back: &Back) -> PloyEdge<Self::Base> {
//         self.as_ref().backed(back)
//     }
//     async fn solve(&self) -> Result<Hub<Self::Base>> {
//         self.as_ref().solve().await
//     }
// }

// impl<T> SetRoot for Box<dyn Engage<Base = T>> {
//     fn set_root(&mut self, root: Root) {
//         self.as_mut().set_root(root);
//     }
// }

// impl<T> Reckon for Box<dyn Engage<Base = T>> {
//     fn reckon(&self, task: Task) -> Result<Gain> {
//         self.as_ref().reckon(task)
//     }
// }

// impl<T> AdaptGet for Box<dyn Engage<Base = T>> {
//     fn adapt_get(&self, deal: &mut dyn Deal) -> Result<()> {
//         self.as_ref().adapt_get(deal)
//     }
// }

// #[cfg_attr(not(feature = "oneThread"), async_trait)]
// #[cfg_attr(feature = "oneThread", async_trait(?Send))]
// impl<T> AdaptSet for Box<dyn Engage<Base = T>> {
//     async fn adapt_set(&self, deal: &mut dyn Deal) -> Result<()> {
//         self.as_ref().adapt_set(deal).await
//     }
// }

// impl<T> AddRoot for Box<dyn Engage<Base = T>> {
//     fn add_root(&self, root: Root) -> Result<()> {
//         self.as_ref().add_root(root)
//     }
// }

// impl<T> Rebut for Box<dyn Engage<Base = T>> {
//     fn rebut(&self) -> Result<Ring> {
//         self.as_ref().rebut()
//     }
//     fn clear_roots(&self) -> Result<()> {
//         self.as_ref().clear_roots()
//     }
// }

// #[cfg_attr(not(feature = "oneThread"), async_trait)]
// #[cfg_attr(feature = "oneThread", async_trait(?Send))]
// impl<T> React for Box<dyn Engage<Base = T>> {
//     async fn react(&self, id: &Id) -> Result<()> {
//         self.as_ref().react(id).await
//     }
// }

// impl<T> ToId for Box<dyn Engage<Base = T>> {
//     fn id(&self) -> Id {
//         self.as_ref().id()
//     }
// }
