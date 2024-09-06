use super::*;

/// `Link` to domain-specific unit.
/// The unit type is erased. To keep unit type intact, use `Node` instead.
pub type Ploy<T> = Link<Box<dyn Engage<Out = T>>, T>;

#[cfg(not(feature = "oneThread"))]
pub type PloyPointer<T> = Arc<RwLock<Box<dyn Engage<Out = T>>>>;
#[cfg(feature = "oneThread")]
pub type PloyPointer = Rc<RefCell<Box<dyn Engage>>>;

/// General engagement of Ploy with erased unit type.
pub trait Engage: AdaptMid + SolvePloy + AddRoot + Update + Debug {
    // type Wow;
}

impl<E> Engage for E where 
    E: AdaptMid + SolvePloy + AddRoot + Update + Debug,
    // <E as BackedPloy>::PloyOut: Solve::Out,
{
    // type Wow = <E as Solve>::Out;
}

pub trait ToPloy {
    type ToPloyOut;
    /// Copy with unit type erased.  
    fn ploy(&self) -> PloyPointer<Self::ToPloyOut>;
}

pub trait SolvePloy {
    type Out: Payload;
    fn backed_ploy(&self, back: &Back) -> PloyPointer<Self::Out>;
    fn solve_ploy(&self, task: Task) -> Result<Gain<Self::Out>>; //  where <Self as solve::Solve>::Out: Payload
}


impl<T> AdaptMid for Box<dyn Engage<Out = T>> {
    fn adapt(&self, deal: &mut dyn Deal) -> Result<()> {
        self.as_ref().adapt(deal)
    }
}

impl<T> SolvePloy for Box<dyn Engage<Out = T>> 
where 
    T: Payload
{
    type Out = T;
    fn backed_ploy(&self, back: &Back) -> PloyPointer<Self::Out> {
        self.as_ref().backed_ploy(back)
    }
    fn solve_ploy(&self, task: Task) -> Result<Gain<Self::Out>> {
        self.as_ref().solve_ploy(task)
    }
}

// impl<T> Solve for Box<dyn Engage<Out = T>> 
// where 
//     T: Payload
// {
//     type Out = T;
//     fn solve(&self, task: Task) -> Result<Gain<T>> {
//         self.as_ref().solve(task)
//     }
// }

impl<T> AddRoot for Box<dyn Engage<Out = T>> {
    fn add_root(&self, root: Root) -> Result<()> {
        self.as_ref().add_root(root)
    }
}

impl<T> ToId for Box<dyn Engage<Out = T>> {
    fn id(&self) -> Id {
        self.as_ref().id()
    }
}

impl<T> Rebut for Box<dyn Engage<Out = T>> {
    fn rebut(&self) -> Result<Ring> {
        self.as_ref().rebut()
    }
}

impl<T> React for Box<dyn Engage<Out = T>> {
    fn react(&self, id: &Id) -> react::Result {
        self.as_ref().react(id)
    }
}

// pub trait ToPipedPloy {
//     /// Copy with unit type erased.
//     fn ploy(&self) -> PloyPointer;
// }
