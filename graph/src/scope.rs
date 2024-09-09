use super::*;
use anyhow::anyhow;

#[derive(Default, Debug)]
pub struct Space {
    id: u64,
    imports: Vec<Import>,
    pub apex: Apex,
    pub map: HashMap<Key, Space>,
    // pub vec: Vec<Space>,
    // path: Vec<Key>,
}

impl Space {
    pub fn new(path: Vec<Key>, apex: impl Into<Apex>) -> Self {
        let apex: Apex = apex.into();
        let mut space = Self {
            id: rand::random(),
            apex: apex.clone(),
            // path: path.clone(),
            ..Default::default()
        };
        if let Ok(imports) = apex.imports() {
            space.imports = imports;
        }
        if let Ok(stems) = apex.all() {
            for (key, next_apex) in &stems {
                if next_apex.tray_path().is_some() {
                    continue;
                }
                let mut next_path = path.clone();
                next_path.push(key.clone());
                let next_scope = Self::new(next_path, next_apex);
                space.map.insert(key.clone(), next_scope);
                // if let Some(Path::Local(keys)) = next_apex.path() {
                //     let key = keys.first().expect("No keys in path.");
                //     next_path.push(key.clone());
                //     let next_scope = Self::new(next_path, next_apex);
                //     space.map.insert(key.clone(), next_scope);
                // } else {
                //     space.vec.push(Self::new(next_path, next_apex));
                // }
            }
        }
        space
    }
    pub fn get(&self, keys: &[Key]) -> Result<Apex> {
        if keys.is_empty() {
            Ok(self.apex.clone())
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
    pub fn deal(&self, view: View) -> Result<()> {
        if let Some(Path::Local(keys)) = view.tray_path() {
            if let Ok(rhs) = self.local.get(keys) {
                if let Some(back) = self.back.as_ref() {
                    view.set(rhs.backed(back)?)?;
                } else {
                    return no_back("Scope");
                }
            } else if self.local.imports.contains(&WORLD_ALL) {
                if let Ok(rhs) = self.world.get(keys) {
                    if let Some(back) = self.back.as_ref() {
                        view.set(rhs.backed(back)?)?;
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
    fn one(&mut self, _: &str, view: View) -> Result<()> {
        self.deal(view)
    }
    fn vec<'a>(&mut self, _: &str, view: ViewVec) -> Result<()> {
        for view in view.views() {
            self.deal(view)?;
        }
        Ok(())
    }
    fn map(&mut self, map: &mut Map) -> Result<()> {
        for (_, apex) in map.iter_mut() {
            self.deal(apex.into())?;
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
