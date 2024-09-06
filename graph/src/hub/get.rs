use super::*;

impl<T> Hub<T> 
where 
    T: Payload
{
    /// Get one hub.
    pub fn get(&self, aim: impl Into<Aim>) -> Result<Apex> {
        match self {
            Self::Ploy(ploy) => {
                let mut get = Get::new(aim.into());
                ploy.adapt(&mut get)?;
                get.hub()
            }
            _ => Err(Error::NotPloy)?,
        }
    }
    /// Get vector of all hubes.
    pub fn all(&self) -> Result<Vec<Apex>> {
        match self {
            Self::Ploy(ploy) => {
                let mut all = All { apexes: vec![] };
                ploy.adapt(&mut all)?;
                Ok(all.apexes)
            }
            _ => Err(Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
pub struct Get {
    aim: Aim,
    apex: Option<Apex>,
    errors: Vec<aim::Error>,
}

impl Get {
    pub fn new(aim: Aim) -> Self {
        Self {
            aim,
            apex: None,
            errors: vec![],
        }
    }
    pub fn hub(self) -> Result<Apex> {
        if let Some(hub) = self.apex {
            Ok(hub)
        } else {
            Err(hub::Error::NotFound(self.errors))?
        }
    }
}

impl Deal for Get {
    fn one<'a>(&mut self, key: &str, view: View<'a>) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(rhs) => {
                if key == rhs {
                    self.apex = Some(view.apex());
                }
            }
            _ => self.errors.push(self.aim.wrong_variant("Key")),
        }
        Ok(())
    }
    fn vec<'a>(&mut self, _: &str, views: Vec<View<'a>>) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match self.aim {
            Aim::Index(i) => {
                if i < views.len() {
                    self.apex = Some(views[i].apex());
                } else {
                    self.errors.push(self.aim.index_out_of_bounds(views.len()));
                }
            }
            _ => self.errors.push(self.aim.wrong_variant("Index")),
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(key) => {
                self.apex = map.get(key);
            }
            _ => self.errors.push(self.aim.wrong_variant("Key")),
        }
        Ok(())
    }
}

#[derive(Debug)]
struct All {
    apexes: Vec<Apex>,
}

impl Deal for All {
    fn one<'a>(&mut self, _: &str, view: View<'a>) -> Result<()> {
        self.apexes.push(view.apex());
        Ok(())
    }
    fn vec<'a>(&mut self, _: &str, views: Vec<View<'a>>) -> Result<()> {
        for view in views {
            self.apexes.push(view.apex())
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        self.apexes.extend(map.all());
        Ok(())
    }
}
