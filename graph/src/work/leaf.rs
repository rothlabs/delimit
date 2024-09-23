use crate::*;

/// Work that holds a base. The most simple work that allows read, write, and copy of the base.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Leaf<T> {
    base: T,
    #[serde(skip)]
    digest: Option<u64>,
}

impl<T: Payload> Leaf<T> {
    pub fn new(tray: T) -> Self {
        Self {
            base: tray,
            digest: None,
        }
    }
    pub fn hub(self) -> Hub<T> {
        Hub::Leaf(link::Leaf::new(self.base))
    }
}

impl<T: Payload> Leaf<T> {
    fn digest(&mut self) -> Result<Gain> {
        if let Some(digest) = &self.digest {
            digest.gain()
        } else {
            let mut state = DefaultHasher::new();
            self.base.hash(&mut state);
            let digest = state.finish();
            self.digest = Some(digest);
            digest.gain()
        }
    }
}

impl<T> WorkFromBase for Leaf<T> {
    type Base = T;
    fn from_base(base: Self::Base) -> Self {
        Self { base, digest: None }
    }
}

impl<T> ToItem for Leaf<T> {
    type Item = T;
    fn item(&self) -> &Self::Item {
        &self.base
    }
}

impl<T> BaseMut for Leaf<T> {
    type Base = T;
    fn base(&mut self) -> &mut T {
        &mut self.base
    }
}

impl<T> ReactMut for Leaf<T> {
    fn react_mut<'a>(&'a mut self) -> GraphFuture<Result<()>> {
        Box::pin(async move { Ok(()) })
    }
}

impl<T: Payload> SolveMut for Leaf<T> {
    type Base = ();
    // fn solve<'a>(&'a mut self) -> Result<Hub<()>> {
    //     solve_ok()
    // }
    fn reckon(&mut self, task: Task) -> Result<Gain> {
        match task {
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => task.no_handler(self),
        }
    }
}

impl<T> Clear for Leaf<T> {
    fn clear(&mut self) {
        self.digest = None;
    }
}
