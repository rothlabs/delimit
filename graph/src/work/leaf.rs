use crate::*;

/// Work that holds a tray. The most simple work that allows read, write, and copy of the tray.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Leaf<T> {
    value: T,
    #[serde(skip)]
    digest: Option<u64>,
}

impl<T> Leaf<T> 
where 
    T: Payload
{
    pub fn new(tray: T) -> Self {
        Self { value: tray, digest: None }
    }
    pub fn hub(self) -> Hub<T> {
        Hub::Leaf(link::Leaf::new(self.value))
    }
}

impl<T> Leaf<T> 
where 
    T: Payload
{
    fn digest(&mut self) -> Result<Gain<T>> {
        if let Some(digest) = &self.digest {
            digest.gain()
        } else {
            let mut state = DefaultHasher::new();
            self.value.hash(&mut state);
            let digest = state.finish();
            self.digest = Some(digest);
            digest.gain()
        }
    }
}

impl<T> FromItem for Leaf<T> {
    type Item = T;
    fn new(tray: Self::Item) -> Self {
        Self { value: tray, digest: None }
    }
}

impl<T> ToItem for Leaf<T> {
    type Item = T;
    fn item(&self) -> &Self::Item {
        &self.value
    }
}

impl<T> MutTray<T> for Leaf<T> {
    fn tray(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> ReactMut for Leaf<T> {
    fn react(&mut self, _: &Id) -> react::Result {
        Ok(())
    }
}

impl<T> SolveMut for Leaf<T> 
where 
    T: Payload, //Hash + Serialize + Debug + SendSync,//Hash + Serialize + Debug
{
    type Out = T;
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