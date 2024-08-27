use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U> {
    imports: Vec<Import>,
    unit: Option<U>,
    main: Option<Gain>,
    digest: Option<Gain>,
    serial: Option<Gain>,
}

impl<U> Node<U>
where
    U: Solve,
{
    fn main(&mut self) -> solve::Result {
        if let Some(main) = &self.main {
            Ok(main.clone())
        } else {
            let main = self.unit.as_ref().unwrap().solve(Task::Main)?;
            self.main = Some(main.clone());
            Ok(main)
        }
    }
    fn digest(&mut self) -> solve::Result {
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
    fn serial(&mut self) -> solve::Result {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            let serial = self.unit.as_ref().unwrap().solve(Task::Serial)?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

impl<U> DoSolve for Node<U>
where
    U: Solve,
{
    fn do_solve(&mut self, task: Task) -> solve::Result {
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
{
    type Unit = U;
    fn make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U> FromSnapMid for Node<U>
where
    U: Adapt + Solve,
{
    type Unit = U;
    fn from_snap(&mut self, snap: Snap<Self::Unit>, back: &Back) {
        self.unit = Some(snap.unit);
        self.unit
            .as_mut()
            .unwrap()
            .adapt(Post::Trade(back))
            .expect("To make Node, unit must Adapt with Post::Trade.");
        self.imports = snap.imports;
    }
}

impl<U> FromItem for Node<U>
where
    U: Solve,
{
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            ..Default::default()
        }
    }
}

impl<U> ReadMid for Node<U>
where
    U: Solve,
{
    type Item = U;
    fn read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Node<U>
where
    U: Solve + Debug,
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
{
    type Unit = U;
    fn write_unit_work<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> T {
        let out = write(&mut Pack {
            unit: self.unit.as_mut().unwrap(),
            back,
        });
        self.main = None;
        out
    }
}

impl<U> ReactMut for Node<U>
where
    U: Solve,
{
    fn react(&mut self, _: &Id) -> react::Result {
        self.unit.as_ref().unwrap().solve(Task::React)?;
        Ok(())
    }
}

impl<U> Adapt for Node<U>
where
    U: Adapt,
{
    fn adapt(&mut self, post: Post) -> adapt::Result {
        self.unit.as_mut().unwrap().adapt(post)
    }
}
