use serde::Serialize;

use crate::*;

/// Contains a unit that must impl Grant to produce a Load which is saved here.
#[derive(Serialize)]
pub struct Deuce<U, L> {
    unit: Option<U>,
    load: Option<L>,
}

impl<U, L> Default for Deuce<U, L> {
    fn default() -> Self {
        Self {
            unit: None,
            load: None,
        }
    }
}

impl<U, L> Make for Deuce<U, L> {
    type Unit = U;
    fn make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U, L> FromItem for Deuce<U, L> {
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            load: None,
        }
    }
}

impl<U, L> Read for Deuce<U, L> {
    type Item = U;
    fn read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U, L> Clear for Deuce<U, L> {
    fn clear(&mut self) {
        self.load = None;
    }
}

impl<U, L> WriteWithBack for Deuce<U, L> {
    type Unit = U;
    fn write_with_back<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, back: &Back) {
        write(&mut Pack {
            unit: self.unit.as_mut().unwrap(),
            back,
        });
        self.load = None;
    }
}

impl<U, L> Grantor for Deuce<U, L>
where
    U: Grant<Load = L>,
    L: Clone,
{
    type Load = L;
    fn grantor(&mut self) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.as_ref().unwrap().grant();
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U, L> React for Deuce<U, L> {
    fn react(&mut self, _: &Meta) {}
}
