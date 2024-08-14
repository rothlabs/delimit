use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be return on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Agent<U> {
    unit: Option<U>,
    gain: Option<Gain>,
}

impl<U> Default for Agent<U>
where
    U: Solve,
{
    fn default() -> Self {
        Self {
            unit: None,
            gain: None,
        }
    }
}

impl<U> MakeInner for Agent<U>
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
            gain: None,
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
        self.gain = None;
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
        self.gain = None;
        out
    }
}

impl<U> DoReact for Agent<U>
where
    U: Solve,
{
    fn do_react(&mut self, _: &Id) -> react::Result {
        self.unit.as_ref().unwrap().solve(Task::React)?;
        Ok(())
    }
}

impl<U> DoSolve for Agent<U>
where
    U: Solve,
{
    fn do_solve(&mut self, task: Task) -> solve::Result {
        if let Task::Main = task {
            if let Some(gain) = &self.gain {
                Ok(gain.clone())
            } else {
                let gain = self.unit.as_ref().unwrap().solve(task)?;
                self.gain = Some(gain.clone());
                Ok(gain)
            }
        } else {
            self.unit.as_ref().unwrap().solve(task)
        }
    }
}

impl<U> Adapt for Agent<U>
where
    U: Adapt,
{
    fn adapt(&mut self, post: Post) -> adapt::Result {
        self.unit.as_mut().unwrap().adapt(post)
    }
}

impl<U> Serialize for Agent<U>
where
    U: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.unit.as_ref().unwrap().serialize(serializer)
    }
}
