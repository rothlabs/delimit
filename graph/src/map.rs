use super::*;

/// Key-Apex map.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map(HashMap<Key, Apex>);

impl Map {
    pub fn insert(&mut self, key: Key, apex: Apex) {
        self.0.insert(key, apex);
    }
    pub fn extend(&mut self, other: Map) {
        self.0.extend(other.0);
    }
    pub fn trade(&self, deal: &dyn Trade) -> Self {
        let mut map = HashMap::new();
        for (key, apex) in &self.0 {
            map.insert(key.clone(), apex.deal(deal));
        }
        Map(map)
    }
    pub fn get(&self, key: &Key) -> Option<Apex> {
        self.0.get(key).map(|apex| apex.pathed(key))
    }
    pub fn vec(&self) -> Vec<Apex> {
        let mut out = vec![];
        for (key, apex) in &self.0 {
            out.push(apex.pathed(key));
        }
        out
        // self.0.values().cloned().collect()
    }
    pub fn iter(&self) -> Iter<String, Apex> {
        self.0.iter()
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.0.iter().collect();
        pairs.sort_by_key(|i| i.0);
        Hash::hash(&pairs, state);
    }
}
