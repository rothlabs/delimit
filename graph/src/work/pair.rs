use serde::Serialize;

use crate::*;

/// Contains a unit that must impl Grant to produce a Load which is saved here.
#[derive(Serialize)]
pub struct Pair<U, L> {
    unit: U,
    load: Option<L>,
}

impl<U, L> FromItem for Pair<U, L> {
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self { unit, load: None }
    }
}

impl<U, L> Read for Pair<U, L> {
    type Unit = U;
    fn read(&self) -> &Self::Unit {
        &self.unit
    }
}

impl<U, L> Clear for Pair<U, L> {
    fn clear(&mut self) {
        self.load = None;
    }
}

impl<U, L> WriteWithRoot for Pair<U, L> {
    type Unit = U;
    fn write_with_root<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, root: &Back) {
        write(&mut Pack {
            unit: &mut self.unit,
            back: root,
        });
        self.load = None;
    }
}

impl<U, L> Grantor for Pair<U, L>
where
    U: Grant<Load = L>,
    L: Clone,
{
    type Load = L;
    fn grantor(&mut self) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.grant();
            self.load = Some(load.clone());
            load
        }
    }
}
