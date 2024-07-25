// use serde::Serialize;

use crate::*;

/// Contains a unit that must impl Grant to produce a Load which is saved here.
// #[derive(Serialize)]
pub struct Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    unit: Option<U>,
    midd: Option<U::Load>,
    load: Option<<U::Load as Grant>::Load>,
}

impl<U> Default for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    fn default() -> Self {
        Self {
            unit: None,
            midd: None,
            load: None,
        }
    }
}

impl<U> DoMake for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    type Unit = U;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U> FromItem for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            midd: None,
            load: None,
        }
    }
}

impl<U> DoRead for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    type Item = U;
    fn do_read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    fn clear(&mut self) {
        // TODO: Check if midd should be cleared
        self.midd = None;
        self.load = None;
    }
}

impl<U> WriteUnitWork for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
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
        // TODO: Check if midd should be cleared
        self.midd = None;
        self.load = None;
        out
    }
}

impl<U> DoGrant for Pipe<U>
where
    U: Grant,
    U::Load: Grant + Backed,
    <U::Load as Grant>::Load: Clone,
{
    type Load = <U::Load as Grant>::Load;
    fn do_grant(&mut self, back: &Back) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let midd = self.unit.as_ref().unwrap().grant().backed(back);
            let load = midd.grant();
            self.midd = Some(midd);
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U> DoReact for Pipe<U>
where
    U: Grant,
    U::Load: Grant,
{
    fn do_react(&mut self, _: &Meta) -> ReactResult {
        Ok(())
    }
}
