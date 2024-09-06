use super::*;

impl<T> Hub<T> 
where 
    T: Payload
{
    /// Set one hub.
    pub fn set(&self, aim: impl Into<Aim>, hub: impl Into<Hub<T>>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Set::new(aim, hub)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
    // Insert one hub
    pub fn insert(&self, aim: impl Into<Aim>, hub: impl Into<Hub<T>>) -> Result<()> {
        match self {
            Self::Ploy(ploy) => ploy.adapt(&mut Insert::new(aim, hub)),
            _ => Err(hub::Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
struct Set<T> 
where 
    T: 'static + Payload
{
    aim: Aim,
    apex: Hub<T>,
    back: Option<Back>,
    wrote: bool,
}

impl<T> Set<T> 
where 
    T: Payload
{
    fn new(aim: impl Into<Aim>, hub: impl Into<Hub<T>>) -> Self {
        Self {
            aim: aim.into(),
            hub: hub.into(),
            back: None,
            wrote: false,
        }
    }
}

impl<T> Deal for Set<T> 
where 
    T: Payload
{
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
                    //hubes[i] = self.hub.backed(back)?;
                    views[i].set(apex);
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
struct Insert<T>
where 
T: 'static + Payload
{
    aim: Aim,
    hub: Hub<T>,
    back: Option<Back>,
    wrote: bool,
}

impl<T> Insert<T> 
where 
    T: Payload
{
    fn new(aim: impl Into<Aim>, hub: impl Into<Hub<T>>) -> Self {
        Self {
            aim: aim.into(),
            hub: hub.into(),
            back: None,
            wrote: false,
        }
    }
}

impl<T> Deal for Insert<T> 
where 
    T: Payload
{
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
