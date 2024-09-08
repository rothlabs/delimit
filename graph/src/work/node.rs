use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U: Solve>
where
    U::Out: 'static + Payload,
{
    imports: Vec<Import>,
    unit: Option<U>,
    main: Option<Gain<U::Out>>,
    digest: Option<Gain<U::Out>>,
    serial: Option<Gain<U::Out>>,
}

impl<U> Node<U>
where
    U: Solve,
    U::Out: Payload,
{
    fn main(&mut self) -> Result<Gain<U::Out>> {
        if let Some(main) = &self.main {
            Ok(main.clone())
        } else {
            let main = self.unit.as_ref().unwrap().solve(Task::Main)?;
            self.main = Some(main.clone());
            Ok(main)
        }
    }
    fn digest(&mut self) -> Result<Gain<U::Out>> {
        if let Some(digest) = &self.digest {
            Ok(digest.clone())
        } else {
            let mut state = UnitHasher::default();
            self.imports.hash(&mut state);
            let digest = self
                .unit
                .as_ref()
                .unwrap()
                .solve(Task::Digest(&mut state))?;
            self.digest = Some(digest.clone());
            Ok(digest)
        }
    }
    fn serial(&mut self) -> Result<Gain<U::Out>> {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            let serial = self.unit.as_ref().unwrap().solve(Task::Serial)?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

impl<U> SolveMut for Node<U>
where
    U: Solve,
    U::Out: Payload,
{
    type Out = U::Out;
    fn solve(&mut self, task: Task) -> Result<Gain<U::Out>> {
        match task {
            Task::Main => self.main(),
            Task::Hash => self.digest(),
            Task::Serial => self.serial(),
            Task::Imports => self.imports.gain(),
            _ => self.unit.as_ref().unwrap().solve(task),
        }
    }
}

impl<U> Default for Node<U>
where
    U: Solve,
    U::Out: Payload,
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

impl<U> MakeMid for Node<U>
where
    U: Solve,
    U::Out: Payload,
{
    type Unit = U;
    fn make<F: FnOnce(&Back) -> Result<Self::Unit>>(
        &mut self,
        make: F,
        back: &Back,
    ) -> Result<Option<u64>> {
        self.unit = Some(make(back)?);
        Ok(if let Ok(Gain::U64(rank)) = self.solve(Task::Rank) {
            Some(rank)
        } else {
            None
        })
    }
}

impl<U> WithSnap for Node<U>
where
    U: Adapt + Solve,
    U::Out: Payload,
{
    type Unit = U;
    fn with_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) -> Option<u64> {
        self.unit = Some(snap.unit);
        self.unit
            .as_mut()
            .unwrap()
            .adapt(&mut back.clone())
            .expect("To make Node, unit must Adapt with Post::Trade.");
        self.imports = snap.imports;
        if let Ok(Gain::U64(rank)) = self.solve(Task::Rank) {
            Some(rank)
        } else {
            None
        }
    }
}

impl<U> FromItem for Node<U>
where
    U: Solve,
    U::Out: Payload,
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
    U::Out: Payload,
{
    type Item = U;
    fn item(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Node<U>
where
    U: Solve,
    U::Out: Payload,
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
    U::Out: Payload,
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
    U::Out: Payload,
{
    fn react(&mut self, _: &Id) -> react::Result {
        self.unit.as_ref().unwrap().solve(Task::React)?;
        Ok(())
    }
}

impl<U> Adapt for Node<U>
where
    U: Adapt + Solve,
    U::Out: Payload,
{
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.as_mut().unwrap().adapt(deal)
    }
}
