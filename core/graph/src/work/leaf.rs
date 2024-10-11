use crate::*;

/// Work that holds a base. The most simple work that allows read, write, and copy of the base.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Leaf<T> {
    base: T,
    #[serde(skip)]
    digest: Option<u64>,
    serial: Option<String>,
}

impl<T: Payload> Leaf<T> {
    pub fn new(base: T) -> Self {
        Self {
            base,
            digest: None,
            serial: None,
        }
    }
    pub fn hub(self) -> Hub<T> {
        Hub::Leaf(link::Leaf::new(self.base))
    }
}

// impl<T: Payload> Leaf<T> {
//     fn digest(&mut self) -> u64 {
//         if let Some(digest) = self.digest {
//             digest
//         } else {
//             let mut state = DefaultHasher::new();
//             self.base.digest(&mut state);
//             let digest = state.finish();
//             self.digest = Some(digest);
//             digest
//         }
//     }
// }

impl<T> WorkFromBase for Leaf<T> {
    type Base = T;
    fn from_base(base: Self::Base) -> Self {
        Self {
            base,
            digest: None,
            serial: None,
        }
    }
}

impl<T> ToItem for Leaf<T> {
    type Item = T;
    fn item(&self) -> &Self::Item {
        &self.base
    }
}

impl<T> BaseMut for Leaf<T> {
    type Base = T;
    fn base(&mut self) -> &mut T {
        &mut self.base
    }
}

// impl<T: Payload> SolveMut for Leaf<T> {
//     type Base = ();
//     // fn rank(&self) -> u16 {
//     //     0
//     // }
// }

impl<T: Payload> ReckonMut for Leaf<T> {
    fn get_imports(&self) -> Result<Vec<Import>> {
        Ok(vec![])
    }
    fn get_hash(&mut self) -> Result<u64> {
        if let Some(digest) = self.digest {
            Ok(digest)
        } else {
            let mut state = DefaultHasher::new();
            self.base.digest(&mut state);
            let digest = state.finish();
            self.digest = Some(digest);
            Ok(digest)
        }
        // Ok(self.digest())
    }
    fn get_serial(&mut self) -> Result<String> {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            // let mut state = DefaultHasher::new();
            // self.base.digest(&mut state);
            let serial = self.serial()?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

impl<T> Clear for Leaf<T> {
    fn clear(&mut self) {
        self.digest = None;
        self.serial = None;
    }
}

impl<T> ReactMut for Leaf<T> {}
