use super::*;

impl<T> Hub<T> 
where 
    T: Payload
{
    /// Set one hub.
    pub fn set(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set::new(aim, apex)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
    // Insert one hub
    pub fn insert(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert::new(aim, apex)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
struct Set {
    aim: Aim,
    apex: Apex,
    back: Option<Back>,
    wrote: bool,
}

impl Set {
    fn new(aim: impl Into<Aim>, apex: impl Into<Apex>) -> Self {
        Self {
            aim: aim.into(),
            apex: apex.into(),
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
    fn vec<'a>(&mut self, _: &str, views: Vec<View>) -> Result<()> {
        if let Some(back) = &self.back {
            match self.aim {
                Aim::Index(i) => {
                    views[i].set(self.apex.backed(back)?);
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
            map.insert(&self.aim, self.apex.backed(back)?)
        } else {
            no_back("Set")
        }
    }
}

#[derive(Debug)]
struct Insert {
    aim: Aim,
    apex: Apex,
    back: Option<Back>,
    wrote: bool,
}

impl Insert {
    fn new(aim: impl Into<Aim>, apex: impl Into<Apex>) -> Self {
        Self {
            aim: aim.into(),
            apex: apex.into(),
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
            map.insert(&self.aim, self.apex.backed(back)?)
        } else {
            no_back("Insert")
        }
    }
}
