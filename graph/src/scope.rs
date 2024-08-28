use super::*;
use anyhow::anyhow;

#[derive(Debug)]
pub struct Scope<'a> {
    pub world: &'a Space,
    pub local: &'a Space,
}

#[derive(Default, Debug)]
pub struct Space {
    id: u64,
    pub apex: Apex,
    // path: Vec<Key>,
    imports: Vec<Import>,
    pub map: HashMap<Key, Space>,
    pub vec: Vec<Space>,
}

impl Space {
    pub fn new(path: Vec<Key>, apex: &Apex) -> Self {
        let mut space = Self {
            id: rand::random(),
            apex: apex.clone(),
            // path: path.clone(),
            ..Default::default()
        };
        if let Ok(imports) = apex.imports() {
            space.imports = imports;
        }
        if let Ok(stems) = apex.stems() {
            for next_apex in &stems {
                if let Apex::Tray(Tray::Path(_)) = next_apex {
                    continue;
                }
                let mut next_path = path.clone();
                if let Some(Path::Local(keys)) = next_apex.path() {
                    let key = keys.first().expect("No keys in path.");
                    next_path.push(key.clone());
                    let next_scope = Self::new(next_path, next_apex);
                    space.map.insert(key.clone(), next_scope);
                } else {
                    space.vec.push(Self::new(next_path, next_apex));
                }
            }
        }
        space
    }
    pub fn get(&self, keys: &[Key]) -> GraphResult<Apex> {
        if keys.is_empty() {
            Ok(self.apex.clone())
        } else if let Some(stem) = self.map.get(&keys[0]) {
            stem.get(&keys[1..])
        } else {
            Err(anyhow!("Entry not found."))?
        }
    }
}

impl Trade for Scope<'_> {
    fn trade(&self, apex: &Apex) -> Apex {
        if let Apex::Tray(Tray::Path(Path::Local(keys))) = apex {
            if let Ok(apex) = self.local.get(keys) {
                return apex;
            } else if self.local.imports.contains(&WORLD_ALL) {
                if let Ok(apex) = self.world.get(keys) {
                    return apex;
                }
            }
        }
        apex.clone()
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

// fn import(&mut self, apex: &Apex) -> Result<(), Error> {
//     if let Ok(imports) = apex.imports() {
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
//     self.apexes.extend(scope.apexes.clone());
// }
// fn upper(&self, rank: usize) -> Result<Rc<Self>, Error> {
//     let scope = self.root.as_ref().ok_or("No root of scope.")?;
//     if rank > 1 {
//         scope.upper(rank - 1)
//     } else {
//         Ok(scope.clone())
//     }
// }
