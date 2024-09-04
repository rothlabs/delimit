use super::*;

impl Apex {
    /// Set one apex.
    pub fn set(&self, aim: impl Into<Aim<'static>>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set::new(aim.into(), &apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    // Insert one apex
    pub fn insert(&self, aim: impl Into<Aim<'static>>, apex: impl Into<Apex>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert::new(aim.into(), &apex.into())),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
struct Set<'a> {
    aim: Aim<'a>,
    apex: &'a Apex,
    // TODO: find way to hold ref to back
    back: Option<Back>,
}

impl<'a> Set<'a> {
    fn new(aim: Aim<'a>, apex: &'a Apex) -> Self {
        Self {
            aim,
            apex,
            back: None,
        }
    }
}

impl Deal for Set<'_> {
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        match self.aim {
            Aim::Index(i) => {
                apexes[i] = self
                    .apex
                    .backed(self.back.as_ref().expect("no back in set!"));
                Ok(())
            }
            _ => Err(self.aim.wrong_variant("Index"))?,
        }
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        map.insert(
            self.aim.clone(),
            self.apex
                .backed(self.back.as_ref().expect("no back in insert!")),
        )
    }
}

#[derive(Debug)]
struct Insert<'a> {
    aim: Aim<'a>,
    apex: &'a Apex,
    back: Option<Back>,
}

impl<'a> Insert<'a> {
    fn new(aim: Aim<'a>, apex: &'a Apex) -> Self {
        Self {
            aim,
            apex,
            back: None,
        }
    }
}

impl<'a> Deal for Insert<'a> {
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        map.insert(
            self.aim.clone(),
            self.apex
                .backed(self.back.as_ref().expect("no back in insert!")),
        )
    }
}
