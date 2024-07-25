use crate::*;

/// Contains a unit that must impl Act to produce a Load which is saved here.
pub struct Agent<U>
where
    U: Act,
{
    unit: Option<U>,
    load: Option<U::Load>,
}

impl<U> Default for Agent<U>
where
    U: Act,
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
    U: Act,
{
    type Unit = U;
    fn do_make<F: FnOnce(&Back) -> Self::Unit>(&mut self, make: F, back: &Back) {
        self.unit = Some(make(back));
    }
}

impl<U> FromItem for Agent<U>
where
    U: Act,
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
    U: Act,
{
    type Item = U;
    fn do_read(&self) -> &Self::Item {
        self.unit.as_ref().unwrap()
    }
}

impl<U> Clear for Agent<U>
where
    U: Act,
{
    fn clear(&mut self) {
        self.load = None;
    }
}

impl<U> WriteUnitWork for Agent<U>
where
    U: Act,
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

impl<U> DoAct for Agent<U>
where
    U: Act,
    U::Load: Clone,
{
    type Load = U::Load;
    fn do_act(&mut self, _: &Back) -> Self::Load {
        if let Some(load) = &self.load {
            load.clone()
        } else {
            let load = self.unit.as_ref().unwrap().act();
            self.load = Some(load.clone());
            load
        }
    }
}

impl<U> DoReact for Agent<U>
where
    U: Act + React,
{
    fn do_react(&mut self, meta: &Meta) -> ReactResult {
        self.unit.as_ref().unwrap().react(meta)
    }
}
