use super::*;

impl Hub {
    /// Get one hub.
    pub fn get(&self, aim: impl Into<Aim>) -> Result<Hub> {
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
    pub fn all(&self) -> Result<Vec<Hub>> {
        match self {
            Self::Ploy(ploy) => {
                let mut all = All { hubes: vec![] };
                ploy.adapt(&mut all)?;
                Ok(all.hubes)
            }
            _ => Err(Error::NotPloy)?,
        }
    }
}

#[derive(Debug)]
pub struct Get {
    aim: Aim,
    hub: Option<Hub>,
    errors: Vec<aim::Error>,
}

impl Get {
    pub fn new(aim: Aim) -> Self {
        Self {
            aim,
            hub: None,
            errors: vec![],
        }
    }
    pub fn hub(self) -> Result<Hub> {
        if let Some(hub) = self.hub {
            Ok(hub)
        } else {
            Err(hub::Error::NotFound(self.errors))?
        }
    }
}

impl Deal for Get {
    fn one(&mut self, key: &str, hub: &mut Hub) -> Result<()> {
        if self.hub.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(rhs) => {
                if key == rhs {
                    self.hub = Some(hub.clone());
                }
            }
            _ => self.errors.push(self.aim.wrong_variant("Key")),
        }
        Ok(())
    }
    fn vec(&mut self, _: &str, hubes: &mut Vec<Hub>) -> Result<()> {
        if self.hub.is_some() {
            return Ok(());
        }
        match self.aim {
            Aim::Index(i) => {
                if i < hubes.len() {
                    self.hub = Some(hubes[i].clone());
                } else {
                    self.errors.push(self.aim.index_out_of_bounds(hubes.len()));
                }
            }
            _ => self.errors.push(self.aim.wrong_variant("Index")),
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        if self.hub.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(key) => {
                self.hub = map.get(key);
            }
            _ => self.errors.push(self.aim.wrong_variant("Key")),
        }
        Ok(())
    }
}

#[derive(Debug)]
struct All {
    hubes: Vec<Hub>,
}

impl Deal for All {
    fn one(&mut self, _: &str, hub: &mut Hub) -> Result<()> {
        self.hubes.push(hub.clone());
        Ok(())
    }
    fn vec(&mut self, _: &str, hubes: &mut Vec<Hub>) -> Result<()> {
        self.hubes.extend(hubes.clone());
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        self.hubes.extend(map.all());
        Ok(())
    }
}
