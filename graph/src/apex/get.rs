use super::*;

impl Apex {
    /// Get one apex.
    pub fn get(&self, aim: impl Into<Aim>) -> Result<Apex> {
        match self {
            Self::Ploy(ploy) => {
                let mut get = Get::new(aim.into());
                ploy.adapt(&mut get)?;
                get.apex()
            }
            _ => Err(Error::NotPloy)?,
        }
    }
    /// Get vector of all apexes.
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
    pub fn apex(self) -> Result<Apex> {
        if let Some(apex) = self.apex {
            Ok(apex)
        } else {
            Err(apex::Error::NotFound(self.errors))?
        }
    }
}

impl Deal for Get {
    fn one(&mut self, key: &str, apex: &mut Apex) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(rhs) => {
                if key == rhs {
                    self.apex = Some(apex.clone());
                }
            }
            _ => self.errors.push(self.aim.wrong_variant("Key")),
        }
        Ok(())
    }
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match self.aim {
            Aim::Index(i) => {
                if i < apexes.len() {
                    self.apex = Some(apexes[i].clone());
                } else {
                    self.errors.push(self.aim.index_out_of_bounds(apexes.len()));
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
    fn one(&mut self, _: &str, apex: &mut Apex) -> Result<()> {
        self.apexes.push(apex.clone());
        Ok(())
    }
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        self.apexes.extend(apexes.clone());
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        self.apexes.extend(map.all());
        Ok(())
    }
}
