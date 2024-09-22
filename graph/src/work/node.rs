use async_trait::async_trait;

use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U: Solve>
where
    U::Base: 'static + Payload,
{
    imports: Vec<Import>,
    unit: U,
    main: Option<Hub<U::Base>>,
    digest: Option<Gain>,
    serial: Option<Gain>,
}

impl<U> Node<U>
where
    U: Solve + Reckon,
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
    U::Base: Payload,
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
}

impl<U> ReckonMut for Node<U>
where
    U: Solve + Reckon,
{
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
    U: Solve + Reckon,
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

impl<U> ToItem for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Item = U;
    fn item(&self) -> &Self::Item {
        &self.unit
    }
}

impl<U> Clear for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    fn clear(&mut self) {
        self.main = None;
        self.digest = None;
        self.serial = None;
    }
}

impl<U> WriteUnitWork for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Unit = U;
    fn write_unit_work<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> Result<T> {
        let out = write(&mut Pack {
            unit: &mut self.unit,
            back,
        });
        // TODO: remove this because clear should happen from Rebut?!
        self.clear(); //self.main = None;
        Ok(out)
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
impl<U> ReactMut for Node<U>
where
    U: Solve + Reckon + SendSync,
    U::Base: Payload,
{
    async fn react(&mut self, _: &Id) -> Result<()> {
        // self.unit.as_ref().unwrap().reckon(Task::React)?;
        // Ok(())
        match self.unit.solve().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

// impl<U> Adapt for Node<U>
// where
//     U: Solve,
//     U::Base: Payload,
// {
//     fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
//         self.unit.adapt(deal)
//     }
//     fn back(&mut self, back: &Back) -> Result<()> {
//         self.unit.back(back)
//     }
// }
