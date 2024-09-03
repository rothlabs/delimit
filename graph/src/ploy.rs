use super::*;

/// `Link` to domain-specific unit.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy = Link<Box<dyn Engage>>;

#[cfg(not(feature = "oneThread"))]
pub type PloyPointer = Arc<RwLock<Box<dyn Engage>>>;
#[cfg(feature = "oneThread")]
pub type PloyPointer = Rc<RefCell<Box<dyn Engage>>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: Solve + AdaptMid + BackedPloy + AddRoot + Update + Debug {}

impl<T> Engage for T where T: Solve + AdaptMid + BackedPloy + AddRoot + Update + Debug {}

// impl Ploy {
//     pub fn echo(&mut self, other: Ploy) {
//         self.e
//     }

// }

pub trait ToPloy {
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyPointer;
}

pub trait BackedPloy {
    fn backed_ploy(&self, back: &Back) -> PloyPointer;
}

impl AdaptMid for Box<dyn Engage> {
    fn adapt<'a>(&self, deal: &'a mut dyn Deal<'a>) -> Result<Memo> {
        self.as_ref().adapt(deal)
    }
}

impl Solve for Box<dyn Engage> {
    fn solve(&self, task: Task) -> Result<Gain> {
        self.as_ref().solve(task)
    }
}

impl AddRoot for Box<dyn Engage> {
    fn add_root(&self, root: Root) {
        self.as_ref().add_root(root)
    }
}

impl ToId for Box<dyn Engage> {
    fn id(&self) -> Id {
        self.as_ref().id()
    }
}

impl Rebut for Box<dyn Engage> {
    fn rebut(&self) -> Ring {
        self.as_ref().rebut()
    }
}

impl React for Box<dyn Engage> {
    fn react(&self, id: &Id) -> react::Result {
        self.as_ref().react(id)
    }
}

// pub trait ToPipedPloy {
//     /// Copy with unit type erased.
//     fn ploy(&self) -> PloyPointer;
// }
