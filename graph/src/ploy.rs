use super::*;

/// `Link` to domain-specific unit.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<Box<dyn Engage<Base = T>>>;

#[cfg(not(feature = "oneThread"))]
pub type PloyPointer<T> = Arc<RwLock<Box<dyn Engage<Base = T>>>>;
#[cfg(feature = "oneThread")]
pub type PloyPointer<T> = Rc<RefCell<Box<dyn Engage<Base = T>>>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Based + AdaptMid + AddRoot + Update + Debug {}

impl<E> Engage for E where E: Based + AdaptMid + AddRoot + Update + Debug {}

pub trait ToPloy {
    type ToPloyOut;
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyPointer<Self::ToPloyOut>;
}

pub trait Based {
    type Base: Payload;
    fn solve(&self, task: Task) -> Result<Gain<Self::Base>>;
    fn backed(&self, back: &Back) -> PloyPointer<Self::Base>;
}

impl<T> AdaptMid for Box<dyn Engage<Base = T>> {
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        self.as_ref().adapt(deal)
    }
}

impl<T> Based for Box<dyn Engage<Base = T>>
where
    T: Payload,
{
    type Base = T;
    fn backed(&self, back: &Back) -> PloyPointer<Self::Base> {
        self.as_ref().backed(back)
    }
    fn solve(&self, task: Task) -> Result<Gain<Self::Base>> {
        self.as_ref().solve(task)
    }
}

impl<T> AddRoot for Box<dyn Engage<Base = T>> {
    fn add_root(&self, root: Root) -> Result<()> {
        self.as_ref().add_root(root)
    }
}

impl<T> ToId for Box<dyn Engage<Base = T>> {
    fn id(&self) -> Id {
        self.as_ref().id()
    }
}

impl<T> Rebut for Box<dyn Engage<Base = T>> {
    fn rebut(&self) -> Result<Ring> {
        self.as_ref().rebut()
    }
}

impl<T> React for Box<dyn Engage<Base = T>> {
    fn react(&self, id: &Id) -> react::Result {
        self.as_ref().react(id)
    }
}