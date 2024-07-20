use crate::*;

/// Contains a unit that must impl Grant to produce a Load which is saved here.
pub struct Deuce<U>
where
    U: Grant,
{
    unit: Option<U>,
    load: Option<U::Load>,
}

impl<U> Default for Deuce<U>
where
    U: Grant,
{
    fn default() -> Self {
        Self {
            unit: None,
            load: None,
        }
    }
}

impl<U> DoMake for Deuce<U>
where
    U: Grant,
{
    type Unit = U;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U> FromItem for Deuce<U>
where
    U: Grant,
{
    type Item = U;
    fn new(unit: Self::Item) -> Self {
        Self {
            unit: Some(unit),
            load: None,
        }
    }
}

impl<U> DoRead for Deuce<U>
where
    U: Grant,
{
    type Item = U;
    fn do_read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Deuce<U>
where
    U: Grant,
{
    fn clear(&mut self) {
        self.load = None;
    }
}

impl<U> WriteWithBack for Deuce<U>
where
    U: Grant,
{
    type Unit = U;
    fn write_with_back<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, back: &Back) {
        write(&mut Pack {
            unit: self.unit.as_mut().unwrap(),
            back,
        });
        self.load = None;
    }
}

impl<U> DoGrant for Deuce<U>
where
    U: Grant,
    U::Load: Clone,
{
    type Load = U::Load;
    fn do_grant(&mut self, _: &Back) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.as_ref().unwrap().grant();
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U> DoReact for Deuce<U>
where
    U: Grant,
{
    fn do_react(&mut self, _: &Meta) {}
}
