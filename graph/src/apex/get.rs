use super::*;

#[derive(Debug)]
pub struct Get<'a> {
    aim: Aim<'a>,
    apex: Option<Apex>,
}

impl<'a> Get<'a> {
    pub fn new(aim: Aim<'a>) -> Self {
        Self { aim, apex: None }
    }
    pub fn apex(self) -> Result<Apex> {
        if let Some(apex) = self.apex {
            Ok(apex)
        } else {
            Err(anyhow!("apex not there"))?
        }
    }
}

impl<'a> Deal for Get<'a> {
    fn one(&mut self, key: &str, apex: &mut Apex) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(k) => {
                if key == k {
                    self.apex = Some(apex.clone());
                }
                Ok(())
            }
            _ => Err(self.aim.wrong_variant("Key"))?,
        }
    }
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match self.aim {
            Aim::Index(i) => {
                if i < apexes.len() {
                    self.apex = Some(apexes[i].clone());
                    Ok(())
                } else {
                    Err(self.aim.index_out_of_bounds(i))?
                }
            }
            _ => Err(self.aim.wrong_variant("Index"))?,
        }
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        if self.apex.is_some() {
            return Ok(());
        }
        match &self.aim {
            Aim::Key(key) => {
                self.apex = map.get(key);
                Ok(())
            }
            _ => Err(self.aim.wrong_variant("Key"))?,
        }
    }
}
