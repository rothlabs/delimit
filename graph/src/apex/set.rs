use super::*;

impl Apex {
    /// Set one apex.
    pub fn set(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set::new(aim.into(), apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    // Insert one apex
    pub fn insert(&self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert::new(aim.into(), apex.into())),
            _ => Err(apex::Error::NotPloy)?,
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
    fn new(aim: Aim, apex: Apex) -> Self {
        Self {
            aim,
            apex,
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
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        if let Some(back) = &self.back {
            match self.aim {
                Aim::Index(i) => {
                    apexes[i] = self.apex.backed(back)?;
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
            map.insert(self.aim.clone(), self.apex.backed(back)?)
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
    fn new(aim: Aim, apex: Apex) -> Self {
        Self {
            aim,
            apex,
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
            map.insert(
                self.aim.clone(),
                self.apex.backed(back)?,
            )?;
            self.wrote = true;
            Ok(())
        } else {
            no_back("Insert")
        }
    }
}
