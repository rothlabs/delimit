use super::*;
use anyhow::anyhow;

#[derive(Default, Debug)]
pub struct Space {
    id: u64,
    imports: Vec<Import>,
    pub hub: Hub,
    pub map: HashMap<Key, Space>,
    pub vec: Vec<Space>,
    // path: Vec<Key>,
}

impl Space {
    pub fn new(path: Vec<Key>, hub: &Hub) -> Self {
        let mut space = Self {
            id: rand::random(),
            hub: hub.clone(),
            // path: path.clone(),
            ..Default::default()
        };
        if let Ok(imports) = hub.imports() {
            space.imports = imports;
        }
        if let Ok(stems) = hub.all() {
            for next_hub in &stems {
                if let Hub::Tray(Tray::Path(_)) = next_hub {
                    continue;
                }
                let mut next_path = path.clone();
                if let Some(Path::Local(keys)) = next_hub.path() {
                    let key = keys.first().expect("No keys in path.");
                    next_path.push(key.clone());
                    let next_scope = Self::new(next_path, next_hub);
                    space.map.insert(key.clone(), next_scope);
                } else {
                    space.vec.push(Self::new(next_path, next_hub));
                }
            }
        }
        space
    }
    pub fn get(&self, keys: &[Key]) -> Result<Hub> {
        if keys.is_empty() {
            Ok(self.hub.clone())
        } else if let Some(stem) = self.map.get(&keys[0]) {
            stem.get(&keys[1..])
        } else {
            Err(anyhow!("Entry not found."))?
        }
    }
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub world: &'a Space,
    pub local: &'a Space,
    pub back: Option<Back>,
}

impl Scope<'_> {
    pub fn deal(&self, hub: &mut Hub) -> Result<()> {
        if let Hub::Tray(Tray::Path(Path::Local(keys))) = hub {
            if let Ok(rhs) = self.local.get(keys) {
                if let Some(back) = self.back.as_ref() {
                    *hub = rhs.backed(back)?;
                } else {
                    return no_back("Scope");
                }
            } else if self.local.imports.contains(&WORLD_ALL) {
                if let Ok(rhs) = self.world.get(keys) {
                    if let Some(back) = self.back.as_ref() {
                        *hub = rhs.backed(back)?;
                    } else {
                        return no_back("Scope");
                    }
                }
            }
        }
        Ok(())
    }
}

impl Deal for Scope<'_> {
    fn back(&mut self, back: &Back) {
        self.back = Some(back.clone());
    }
    fn one(&mut self, _: &str, hub: &mut Hub) -> Result<()> {
        self.deal(hub)?;
        Ok(())
    }
    fn vec(&mut self, _: &str, hubes: &mut Vec<Hub>) -> Result<()> {
        for hub in hubes {
            self.deal(hub)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        for (_, hub) in map.iter_mut() {
            self.deal(hub)?;
        }
        Ok(())
    }
}

impl Hash for Space {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// let mut world_all = false;
// for import in &self.local.imports {
//     if let Import::World(stem) = import {
//         if let meta::Stem::All = stem {
//             world_all = true;
//             eprintln!("import world all");
//         }
//     }
// }

// fn world(scope: &Rc<Space>) -> Result<Rc<Space>, Error> {
//     if let Some(scope) = &scope.root {
//         world(scope)
//     } else {
//         Ok(scope.clone())
//     }
// }

// fn import(&mut self, hub: &Hub) -> Result<(), Error> {
//     if let Ok(imports) = hub.imports() {
//         for import in &imports {
//             match import {
//                 Import::World(stem) => {
//                     //let scope = world(self.root.as_ref().ok_or("No root of scope.")?)?;
//                     match stem {
//                         meta::Stem::All => {
//                             self.extend(scope.as_ref());
//                         }
//                         meta::Stem::Node(_) => {}
//                     }
//                 }
//                 Import::Upper(upper) => {
//                     let scope = self.upper(upper.rank)?;
//                     match &upper.item {
//                         meta::Stem::All => {
//                             eprintln!("importing upper!");
//                             eprintln!("scope: {:?}", scope.as_ref());
//                             self.extend(scope.as_ref());
//                         }
//                         meta::Stem::Node(_) => {}
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }
//     Ok(())
// }
// fn extend(&mut self, scope: &Space) {
//     // self.vec.extend(scope.vec.clone());
//     self.hubes.extend(scope.hubes.clone());
// }
// fn upper(&self, rank: usize) -> Result<Rc<Self>, Error> {
//     let scope = self.root.as_ref().ok_or("No root of scope.")?;
//     if rank > 1 {
//         scope.upper(rank - 1)
//     } else {
//         Ok(scope.clone())
//     }
// }
