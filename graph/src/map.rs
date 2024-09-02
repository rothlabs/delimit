use std::collections::hash_map::{IterMut, Values};

use super::*;

/// Key-Fit map.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map(pub HashMap<Key, Apex>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert<'a>(&mut self, aim: impl Into<Aim<'a>>, apex: impl Into<Apex>) -> Result<Memo> {
        match aim.into() {
            Aim::Key(key) => {
                self.0.insert(key, apex.into());
                adapt_ok()
            }
            aim => Err(adapt::Error::from(aim.wrong_variant("Key")))?
        }
        // if let Aim::Key(key) = aim.into() {
        //     self.0.insert(key, Fit::Apex(apex));
        //     adapt_ok()
        // } else {
        //     Err(adapt::Error::from(aim.into().wrong_variant("Key")))?
        // }
    }
    pub fn extend(&mut self, other: Map) {
        self.0.extend(other.0);
    }
    // pub fn trade(&self, deal: &dyn Trade) -> Self {
    //     let mut map = HashMap::new();
    //     for (key, fit) in &self.0 {
    //         //self.0.entry(key.clone()).or_insert(fit.trade(deal));
    //         map.insert(key.clone(), fit.trade(deal));
    //     }
    //     Map(map)
    // }
    // TODO: use aim instead of key (move aim logic from Apex to Map)
    pub fn get(&self, key: &Key) -> Option<Apex> {
        self.0.get(key).map(|apex| Some(apex.pathed(key)))?
    }
    // pub fn all(&self) -> Vec<Apex> {
    //     let mut out = vec![];
    //     for apex in self.0.values() {
    //         out.push(apex.clone());
    //     }
    //     out
    //     // self.0.values().cloned().collect()
    // }
    pub fn values(&self) -> Values<String, Apex> {
        self.0.values()
    }
    pub fn iter_mut(&mut self) -> IterMut<Key, Apex> {
        self.0.iter_mut()
    }
    pub fn deal(&mut self, deal: &mut dyn Trade) -> Result<Memo> {
        deal.trade_map(self)
    }
    pub fn backed(&mut self, back: &Back) -> Self {
        let mut map = Map::new();
        for (aim, apex) in &self.0 {
            map.insert(aim, apex.backed(back));
        }
        map
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.0.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, state);
    }
}

#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum Fit {
    Apex(Apex),
    Apexes(Vec<Apex>)
}

impl Fit { 
    fn all(&self) -> Vec<Apex> {
        match self {
            Self::Apex(apex) => vec![apex.clone()],
            Self::Apexes(apexes) => apexes.clone()
        }
    }
    fn first(&self) -> Option<&Apex> {
        match self {
            Self::Apex(apex) => Some(apex),
            Self::Apexes(apexes) => apexes.first(),
        }
    }
    // fn trade(&self, deal: &dyn Trade) -> Self {
    //     match self {
    //         Self::Apex(apex) => Self::Apex(apex.deal(deal)),
    //         Self::Apexes(apexes) => Self::Apexes(apexes.deal(deal)),
    //     }
    // }
    pub fn backed(&self, back: &Back) -> Self {
        match self {
            Self::Apex(apex) => Self::Apex(apex.backed(back)),
            Self::Apexes(apexes) => Self::Apexes(apexes.backed(back)),
        }
    }
}

impl From<Apex> for Fit {
    fn from(value: Apex) -> Self {
        Self::Apex(value)
    }
}

impl From<&Apex> for Fit {
    fn from(value: &Apex) -> Self {
        Self::Apex(value.clone())
    }
}

impl From<&Vec<Apex>> for Fit {
    fn from(value: &Vec<Apex>) -> Self {
        Self::Apexes(value.clone())
    }
}
