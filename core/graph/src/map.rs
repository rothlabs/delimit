use super::*;
use std::collections::hash_map::IterMut;

/// Key-Apex map.
#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Map(pub HashMap<Key, Apex>);

impl Map {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, aim: impl Into<Aim>, apex: Apex) -> Result<()> {
        match &aim.into() {
            Aim::Key(key) => {
                self.0.insert(key.clone(), apex);
                Ok(())
            }
            aim => Err(aim.wrong_variant("Key"))?,
        }
    }
    pub fn get(&self, key: &Key) -> Option<Apex> {
        self.0.get(key).cloned()
    }
    pub fn all(&self) -> Vec<Apex> {
        self.0.values().cloned().collect()
    }
    // pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Key, &mut Apex)>
    pub fn iter_mut(&mut self) -> IterMut<Key, Apex> {
        self.0.iter_mut()
    }
    pub fn deal(&mut self, deal: &mut dyn Deal) -> Result<()> {
        deal.map(self)
    }
    pub fn backed(&mut self, back: &Back) -> Result<Self> {
        let mut map = Map::new();
        for (key, apex) in &self.0 {
            map.insert(key, apex.backed(back)?)?;
        }
        Ok(map)
    }
}

impl HashGraph for Map {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        let mut pairs: Vec<_> = self.0.iter().collect();
        pairs.sort_by_key(|i| i.0);
        for (key, apex) in pairs {
            key.hash(state);
            apex.hash_graph(state);
        }
        // HashGraph::hash_graph(&pairs, state);
    }
}

// impl HashGraph for Vec<(&String, &Apex)> {
//     fn hash_graph<H: Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//     }
// }

// impl Hash for Map {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         let mut pairs: Vec<_> = self.0.iter().collect();
//         pairs.sort_by_key(|i| i.0);
//         Hash::hash(&pairs, state);
//     }
// }
