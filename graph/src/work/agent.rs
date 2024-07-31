use crate::*;

/// Contains a unit that must impl Grant to produce a Load which is saved here.
pub struct Agent<U>
where
    U: Solve,
{
    unit: Option<U>,
    load: Option<solve::Result>,
}

impl<U> Default for Agent<U>
where
    U: Solve,
{
    fn default() -> Self {
        Self {
            unit: None,
            load: None,
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
            load: None,
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
        self.load = None;
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
        self.load = None;
        out
    }
}

impl<U> DoSolve for Agent<U>
where
    U: Solve
{
    fn do_solve(&mut self, _: &Back) -> solve::Result {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.as_ref().unwrap().solve();
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U> DoReact for Agent<U>
where
    U: Solve,
{
    fn do_react(&mut self, _: &Meta) -> react::Result {
        self.unit.as_ref().unwrap().solve();
        Ok(())
    }
}

impl<U> InsertMut for Agent<U> 
where 
    U: InsertMut + Solve
{
    fn insert_mut(&mut self, field: &str, node: Node) {
        self.unit.as_mut().unwrap().insert_mut(field, node);
    }
}