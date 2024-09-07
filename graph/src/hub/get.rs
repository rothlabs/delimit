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
    fn one(&mut self, key: &str, view: View) -> Result<()> {
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
    fn vec(&mut self, _: &str, view: ViewVec) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match self.aim {
            Aim::Index(i) => {
                if let Ok(apex) = view.apex(i) {
                    self.apex = Some(apex);
                } else {
                    self.errors.push(self.aim.index_out_of_bounds(view.len()));
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
    fn one(&mut self, _: &str, view: View) -> Result<()> {
        self.apexes.push(view.apex());
        Ok(())
    }
    fn vec(&mut self, _: &str, view: ViewVec) -> Result<()> {
        self.apexes.extend(view.all());
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        self.apexes.extend(map.all());
        Ok(())
    }
}

                // if i < view.len() {
                //     self.apex = Some(view.apex(i));
                // } else {
                //     self.errors.push(self.aim.index_out_of_bounds(view.len()));
                // }
