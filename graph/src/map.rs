use super::*;

/// Key-Fit map.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map(HashMap<Key, Fit>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert<'a>(&mut self, aim: impl Into<Aim<'a>>, fit: impl Into<Fit>) -> Result<Memo> {
        match aim.into() {
            Aim::Key(key) => {
                self.0.insert(key, fit.into());
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
    pub fn trade(&self, deal: &dyn Trade) -> Self {
        let mut map = HashMap::new();
        for (key, fit) in &self.0 {
            //self.0.entry(key.clone()).or_insert(fit.trade(deal));
            map.insert(key.clone(), fit.trade(deal));
        }
        Map(map)
    }
    // TODO: use aim instead of key (move aim logic from Apex to Map)
    pub fn get(&self, key: &Key) -> Option<Apex> {
        self.0.get(key).map(|fit| Some(fit.first()?.pathed(key)))?
    }
    pub fn all(&self) -> Vec<Apex> {
        let mut out = vec![];
        for fit in self.0.values() {
            out.extend(fit.all());
        }
        out
        // self.0.values().cloned().collect()
    }
    // pub fn iter(&self) -> Iter<String, Apex> {
    //     self.0.iter()
    // }
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
    fn trade(&self, deal: &dyn Trade) -> Self {
        match self {
            Self::Apex(apex) => Self::Apex(apex.deal(deal)),
            Self::Apexes(apexes) => Self::Apexes(apexes.deal(deal)),
        }
    }
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
