use super::*;

#[derive(Debug)]
struct Get<'a> {
    aim: &'a Aim<'a>,
    apex: Option<&'a Apex>,
}

impl<'a> Deal<'a> for Get<'a> {
    fn trade_vec(&mut self, _: &str, apexes: &'a mut Vec<Apex>) -> Result<Memo> {
        match self.aim {
            Aim::Index(i) => {
                self.apex = Some(&apexes[*i]);
                adapt_ok()
            },
            _ => Err(self.aim.wrong_variant("Index"))?
        }
    }
}