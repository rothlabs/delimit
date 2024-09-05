use super::*;

impl Hub {
    /// Set one hub.
    pub fn set(&self, aim: impl Into<Aim>, hub: impl Into<Hub>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set::new(aim, hub)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
    // Insert one hub
    pub fn insert(&self, aim: impl Into<Aim>, hub: impl Into<Hub>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert::new(aim, hub)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
struct Set {
    aim: Aim,
    hub: Hub,
    back: Option<Back>,
    wrote: bool,
}

impl Set {
    fn new(aim: impl Into<Aim>, hub: impl Into<Hub>) -> Self {
        Self {
            aim: aim.into(),
            hub: hub.into(),
            back: None,
            wrote: false,
        }
    }
}

impl Deal for Set {
    fn wrote(&self) -> bool {
        self.wrote
    }
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn vec(&mut self, _: &str, hubes: &mut Vec<Hub>) -> Result<()> {
        if let Some(back) = &self.back {
            match self.aim {
                Aim::Index(i) => {
                    hubes[i] = self.hub.backed(back)?;
                    self.wrote = true;
                    Ok(())
                }
                _ => Err(self.aim.wrong_variant("Index"))?,
            }
        } else {
            no_back("Set")
        }
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        if let Some(back) = &self.back {
            map.insert(&self.aim, self.hub.backed(back)?)
        } else {
            no_back("Set")
        }
    }
}

#[derive(Debug)]
struct Insert {
    aim: Aim,
    hub: Hub,
    back: Option<Back>,
    wrote: bool,
}

impl Insert {
    fn new(aim: impl Into<Aim>, hub: impl Into<Hub>) -> Self {
        Self {
            aim: aim.into(),
            hub: hub.into(),
            back: None,
            wrote: false,
        }
    }
}

impl Deal for Insert {
    fn wrote(&self) -> bool {
        self.wrote
    }
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        if let Some(back) = &self.back {
            map.insert(&self.aim, self.hub.backed(back)?)
        } else {
            no_back("Insert")
        }
    }
}
