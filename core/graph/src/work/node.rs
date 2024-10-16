use crate::*;

/// Main Work type.
/// To be useful, unit should at least impl Solve.
/// The solved Gain is kept to be returned on subsequent solve calls
/// until the unit changes.
#[derive(Debug)]
pub struct Node<U: Solve> {
    imports: Vec<Import>,
    unit: U,
    main: Option<Hub<U::Base>>,
    digest: Option<u64>,
    serial: Option<String>,
}

impl<U: Solve + WingOnly> WingOnly for Node<U> {}

impl<U> SolveAdapt for Node<U>
where
    U: Solve + Adapt + IsSend,
    U::Base: Clone
{
    type Base = U::Base;
    fn solve(&mut self) -> GraphFuture<Result<Hub<U::Base>>> {
        Box::pin(async move {
            if let Some(main) = &self.main {
                Ok(main.clone())
            } else {
                let main = self.unit.solve().await?;
                self.main = Some(main.clone());
                Ok(main)
            }
        })
    }
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.unit.adapt(deal)
    }
    fn back(&mut self, back: &Back) -> Result<()> {
        self.unit.back(back)
    }
}

impl<U> ReckonMut for Node<U>
where
    U: Solve + Digest + Serialize,
{
    fn get_imports(&self) -> Result<Vec<Import>> {
        Ok(self.imports.clone())
    }
    fn get_hash(&mut self) -> Result<u64> {
        if let Some(digest) = self.digest {
            Ok(digest)
        } else {
            let mut state = DefaultHasher::new();
            self.unit.digest(&mut state);
            let digest = state.finish();
            self.digest = Some(digest);
            Ok(digest)
        }
    }
    fn get_serial(&mut self) -> Result<String> {
        if let Some(serial) = &self.serial {
            Ok(serial.clone())
        } else {
            let serial = self.unit.serial()?; //self.unit.reckon(Task::Serial)?;
            self.serial = Some(serial.clone());
            Ok(serial)
        }
    }
}

impl<U: Solve> WorkFromSnap for Node<U> {
    type Unit = U;
    fn from_snap(snap: Snap<Self::Unit>) -> (Option<u16>, Self) {
        let node = Self {
            unit: snap.unit,
            imports: snap.imports,
            main: None,
            digest: None,
            serial: None,
        };
        (Some(node.unit.rank()), node)
    }
}

impl<U: Solve> ToItem for Node<U> {
    type Item = U;
    fn item(&self) -> &Self::Item {
        &self.unit
    }
}

impl<U: Solve> Clear for Node<U> {
    fn clear(&mut self) {
        self.main = None;
        self.digest = None;
        self.serial = None;
    }
}

impl<U: Solve> WriteUnitWork for Node<U> {
    type Unit = U;
    fn write_unit_work<T, F>(&mut self, write: F, back: &Back) -> T
    where
        F: FnOnce(&mut Pack<Self::Unit>) -> T,
    {
        write(&mut Pack {
            unit: &mut self.unit,
            back,
        })
    }
}

impl<U> ReactMut for Node<U>
where
    U: Solve + Adapt + IsSend,
    U::Base: Clone
{
    fn react(&mut self) -> GraphFuture<Result<()>> {
        Box::pin(async move {
            match self.solve().await {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        })
    }
}

// impl<U: Solve> Node<U> {
//     fn rank(&self) -> Option<u64> {
//         if let Ok(Gain::U64(rank)) = self.unit.reckon(Task::Rank) {
//             Some(rank)
//         } else {
//             None
//         }
//     }
//     // fn digest(&mut self) -> Result<Gain> {
//     //     if let Some(digest) = &self.digest {
//     //         Ok(digest.clone())
//     //     } else {
//     //         let mut state = UnitHasher::default();
//     //         self.imports.hash(&mut state);
//     //         let digest = self.unit.reckon(Task::Digest(&mut state))?;
//     //         self.digest = Some(digest.clone());
//     //         Ok(digest)
//     //     }
//     // }
//     // fn serial(&mut self) -> Result<Gain> {
//     //     if let Some(serial) = &self.serial {
//     //         Ok(serial.clone())
//     //     } else {
//     //         let serial = self.serial();//self.unit.reckon(Task::Serial)?;
//     //         self.serial = Some(serial.clone());
//     //         Ok(serial)
//     //     }
//     // }
// }
