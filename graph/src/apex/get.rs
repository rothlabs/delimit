use super::*;

#[derive(Debug)]
struct Get<'a> {
    aim: &'a Aim<'a>,
    apex: Option<Apex>,
}

impl<'a> Deal for Get<'a> {
    fn vec(&mut self, _: &str, apexes: &mut Vec<Apex>) -> Result<()> {
        match self.aim {
            Aim::Index(i) => {
                self.apex = Some(apexes[*i].clone());
                Ok(())
            },
            _ => Err(self.aim.wrong_variant("Index"))?
        }
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        match self.aim {
            Aim::Key(key) => {
                self.apex = Some(map.get(key)?.clone());
                Ok(())
            },
            _ => Err(self.aim.wrong_variant("Key"))?
        }
    }
}