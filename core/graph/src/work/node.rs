use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U: Solve> {
    imports: Vec<Import>,
    unit: U,
    main: Option<Hub<U::Base>>,
    digest: Option<Gain>,
    serial: Option<Gain>,
}

impl<U: Solve> Node<U> {
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

impl<U> SolveMut for Node<U>
where
    U: Solve + IsSend,
{
    type Base = U::Base;
    fn solve(&mut self) -> GraphFuture<Result<Hub<U::Base>>> {
        Box::pin(async move {
            if let Some(main) = &self.main {
                Ok(main.clone())
            } else {
                let main = self.unit.solve().await?;
                self.main = Some(main.clone());
                Ok(main)
            }
        })
    }
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.adapt(deal)
    }
    fn back(&mut self, back: &Back) -> Result<()> {
        self.unit.backed(back)
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

impl<U: Solve> WorkFromSnap for Node<U> {
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

impl<U> ReactMut for Node<U>
where
    U: Solve + IsSend,
{
    fn react(&mut self) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            match self.solve().await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        })
    }
}