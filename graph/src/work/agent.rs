use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Tray is kept to be return on subsequent solve calls
/// until the unit changes.
pub struct Agent<U> {
    unit: Option<U>,
    tray: Option<Tray>,
}

impl<U> Default for Agent<U>
where
    U: Solve,
{
    fn default() -> Self {
        Self {
            unit: None,
            tray: None,
        }
    }
}

impl<U> DoMake for Agent<U>
where
    U: Solve,
{
    type Unit = U;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U> FromItem for Agent<U>
where
    U: Solve,
{
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            tray: None,
        }
    }
}

impl<U> DoRead for Agent<U>
where
    U: Solve,
{
    type Item = U;
    fn do_read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Agent<U>
where
    U: Solve,
{
    fn clear(&mut self) {
        self.tray = None;
    }
}

impl<U> WriteUnitWork for Agent<U>
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
        self.tray = None;
        out
    }
}

impl<U> DoReact for Agent<U>
where
    U: Solve,
{
    fn do_react(&mut self, _: &Meta) -> react::Result {
        self.unit.as_ref().unwrap().solve(Task::React)?;
        Ok(())
    }
}

impl<U> DoSolve for Agent<U>
where
    U: Solve,
{
    fn do_solve(&mut self, task: Task, _: &Back) -> solve::Result {
        if let Some(tray) = &self.tray {
            Ok(tray.clone())
        } else {
            let tray = self.unit.as_ref().unwrap().solve(task)?;
            self.tray = Some(tray.clone());
            Ok(tray)
        }
    }
}

impl<U> Alter for Agent<U>
where
    U: Alter,
{
    fn alter(&mut self, post: Post) -> alter::Result {
        self.unit.as_mut().unwrap().alter(post)
    }
}
