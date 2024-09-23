use crate::*;
use async_trait::async_trait;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U>
where
    U: Solve,
{
    imports: Vec<Import>,
    unit: U,
    main: Option<Hub<U::Base>>,
    digest: Option<Gain>,
    serial: Option<Gain>,
}

impl<U> Node<U>
where
    U: Solve,
{
    fn rank(&self) -> Option<u64> {
        if let Ok(Gain::U64(rank)) = self.unit.reckon(Task::Rank) {
            Some(rank)
        } else {
            None
        }
    }
    fn digest(&mut self) -> Result<Gain> {
        if let Some(digest) = &self.digest {
            Ok(digest.clone())
        } else {
            let mut state = UnitHasher::default();
            self.imports.hash(&mut state);
            let digest = self.unit.reckon(Task::Digest(&mut state))?;
            self.digest = Some(digest.clone());
            Ok(digest)
        }
    }
    fn serial(&mut self) -> Result<Gain> {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            let serial = self.unit.reckon(Task::Serial)?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<U> SolveMut for Node<U>
where
    U: Solve + SendSync,
{
    type Base = U::Base;
    async fn solve(&mut self) -> Result<Hub<U::Base>> {
        if let Some(main) = &self.main {
            Ok(main.clone())
        } else {
            let main = self.unit.solve().await?;
            self.main = Some(main.clone());
            Ok(main)
        }
    }
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.adapt(deal)
    }
    fn back(&mut self, back: &Back) -> Result<()> {
        self.unit.back(back)
    }
    fn reckon(&mut self, task: Task) -> Result<Gain> {
        match task {
            Task::Hash => self.digest(),
            Task::Serial => self.serial(),
            Task::Imports => self.imports.gain(),
            _ => self.unit.reckon(task),
        }
    }
}

impl<U> WorkFromSnap for Node<U>
where
    U: Solve,
{
    type Unit = U;
    fn from_snap(snap: Snap<Self::Unit>) -> (Option<u64>, Self) {
        let node = Self {
            unit: snap.unit,
            imports: snap.imports,
            main: None,
            digest: None,
            serial: None,
        };
        (node.rank(), node)
    }
}

impl<U: Solve> ToItem for Node<U> {
    type Item = U;
    fn item(&self) -> &Self::Item {
        &self.unit
    }
}

impl<U: Solve> Clear for Node<U> {
    fn clear(&mut self) {
        self.main = None;
        self.digest = None;
        self.serial = None;
    }
}

impl<U: Solve> WriteUnitWork for Node<U> {
    type Unit = U;
    fn write_unit_work<T, F>(&mut self, write: F, back: &Back) -> T
    where
        F: FnOnce(&mut Pack<Self::Unit>) -> T,
    {
        write(&mut Pack {
            unit: &mut self.unit,
            back,
        })
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<U> ReactMut for Node<U>
where
    U: Solve + SendSync,
{
    async fn react_mut(&mut self) -> Result<()> {
        match self.solve().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
