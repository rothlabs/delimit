use super::*;

impl Apex {
    pub fn set(&self, aim: impl Into<Aim<'static>>, apex: impl Into<Apex>) -> Result<Memo> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set {aim: &aim.into(), apex: &apex.into()}),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    pub fn insert(&self, aim: impl Into<Aim<'static>>, apex: impl Into<Apex>) -> Result<Memo> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert{aim: &aim.into(), apex: &apex.into()}),
            _ => Err(apex::Error::NotPloy)?,
        }
    }
    // pub fn extend(&self, apexes: Map) -> Result<Memo> {
    //     match self {
    //         Self::Ploy(ploy) => ploy.adapt(Post::Extend(apexes)),
    //         _ => Err(apex::Error::NotPloy)?,
    //     }
    // }
}

#[derive(Debug)]
struct Set<'a> {
    aim: &'a Aim<'a>,
    apex: &'a Apex,
}

impl Deal for Set<'_> {
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        match self.aim {
            Aim::Index(i) => {
                apexes[*i] = self.apex.clone();
                Ok(())
            },
            _ => Err(self.aim.wrong_variant("Index"))?
        }
    }
}

#[derive(Debug)]
struct Insert<'a> {
    aim: &'a Aim<'a>,
    apex: &'a Apex,
}

impl Deal for Insert<'_> {
    fn map(&mut self, map: &mut Map) -> Result<()> {
        map.insert(self.aim.clone(), self.apex.clone())
    }
}
