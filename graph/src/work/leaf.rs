use crate::*;

/// Work that holds a base. The most simple work that allows read, write, and copy of the base.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Leaf<T> {
    base: T,
    #[serde(skip)]
    digest: Option<u64>,
}

impl<T> Leaf<T>
where
    T: Payload,
{
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

impl<T> Leaf<T>
where
    T: Payload,
{
    fn digest(&mut self) -> Result<Gain<T>> {
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

impl<T> FromItem for Leaf<T> {
    type Item = T;
    fn new(tray: Self::Item) -> Self {
        Self {
            base: tray,
            digest: None,
        }
    }
}

impl<T> ToItem for Leaf<T> {
    type Item = T;
    fn item(&self) -> &Self::Item {
        &self.base
    }
}

impl<T> BaseMut<T> for Leaf<T> {
    fn base(&mut self) -> &mut T {
        &mut self.base
    }
}

impl<T> ReactMut for Leaf<T> {
    fn react(&mut self, _: &Id) -> react::Result {
        Ok(())
    }
}

impl<T> SolveMut for Leaf<T>
where
    T: Payload,
{
    type Base = T;
    fn solve(&mut self, task: Task) -> Result<Gain<T>> {
        match task {
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => task.no_handler(self),
        }
    }
}

impl<T> RebutMut for Leaf<T> {
    fn rebut(&mut self) -> Result<Ring> {
        Ok(Ring::new())
    }
}

impl<T> Clear for Leaf<T> {
    fn clear(&mut self) {
        self.digest = None;
    }
}

impl<T> Adapt for Leaf<T> {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}
