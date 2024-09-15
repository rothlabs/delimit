use futures::executor::block_on;

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
    unit: Option<U>,
    main: Option<Gain<U::Base>>,
    digest: Option<Gain<U::Base>>,
    serial: Option<Gain<U::Base>>,
}

impl<U> Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    async fn main(&mut self) -> Result<Gain<U::Base>> {
        if let Some(main) = &self.main {
            Ok(main.clone())
        } else {
            let main = self.unit.as_ref().unwrap().solve(Task::Main).await?;
            self.main = Some(main.clone());
            Ok(main)
        }
    }
    async fn digest(&mut self) -> Result<Gain<U::Base>> {
        if let Some(digest) = &self.digest {
            Ok(digest.clone())
        } else {
            let mut state = UnitHasher::default();
            self.imports.hash(&mut state);
            let unit = self.unit.as_ref().unwrap();
            let digest = unit.solve(Task::Digest(&mut state)).await?;
            self.digest = Some(digest.clone());
            Ok(digest)
        }
    }
    async fn serial(&mut self) -> Result<Gain<U::Base>> {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            let unit = self.unit.as_ref().unwrap();
            let serial = unit.solve(Task::Serial).await?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

impl<U> SolveMut for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Base = U::Base;
    async fn solve(&mut self, task: Task<'_>) -> Result<Gain<U::Base>> {
        match task {
            Task::Main => self.main().await,
            Task::Hash => self.digest().await,
            Task::Serial => self.serial().await,
            Task::Imports => self.imports.gain(),
            _ => self.unit.as_ref().unwrap().solve(task).await,
        }
    }
}

impl<U> Default for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    fn default() -> Self {
        Self {
            imports: vec![],
            unit: None,
            main: None,
            digest: None,
            serial: None,
        }
    }
}

impl<U> MakeMut for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Unit = U;
    fn make<F: FnOnce(&Back) -> Result<Self::Unit>>(
        &mut self,
        make: F,
        back: &Back,
    ) -> Result<Option<u64>> {
        self.unit = Some(make(back)?);
        Ok(if let Ok(Gain::U64(rank)) = block_on(self.solve(Task::Rank)) {
            Some(rank)
        } else {
            None
        })
    }
}

impl<U> WithSnap for Node<U>
where
    U: Adapt + Solve,
    U::Base: Payload,
{
    type Unit = U;
    fn with_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) -> Option<u64> {
        self.unit = Some(snap.unit);
        self.unit
            .as_mut()
            .unwrap()
            .adapt(&mut back.clone())
            .expect("Adapt must not fail.");
        self.imports = snap.imports;
        if let Ok(Gain::U64(rank)) = block_on(self.solve(Task::Rank)) {
            Some(rank)
        } else {
            None
        }
    }
}

impl<U> FromItem for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            ..Default::default()
        }
    }
}

impl<U> ToItem for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    type Item = U;
    fn item(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
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
            unit: self.unit.as_mut().unwrap(),
            back,
        });
        self.clear(); //self.main = None;
        Ok(out)
    }
}

impl<U> ReactMut for Node<U>
where
    U: Solve,
    U::Base: Payload,
{
    fn react(&mut self, _: &Id) -> react::Result {
        block_on(self.unit.as_ref().unwrap().solve(Task::React))?;
        Ok(())
    }
}

impl<U> Adapt for Node<U>
where
    U: Solve + Adapt,
    U::Base: Payload,
{
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.as_mut().unwrap().adapt(deal)
    }
}
