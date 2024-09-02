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

impl Trade for Set<'_> {
    fn trade_vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<Memo> {
        match self.aim {
            Aim::Index(i) => apexes[*i] = self.apex.clone(),
            _ => return Err(anyhow!("wrong aim in Trade for Set"))?
        }
        adapt_ok()
    }
}

#[derive(Debug)]
struct Insert<'a> {
    aim: &'a Aim<'a>,
    apex: &'a Apex,
}

impl Trade for Insert<'_> {
    fn trade_map(&mut self, map: &mut Map) -> Result<Memo> {
        map.insert(self.aim.clone(), self.apex.clone())
    }
}
