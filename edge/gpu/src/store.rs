// use super::*;

// // #[derive(Debug, Default)]
// pub struct Allocation {
//     offset: u32,
//     length: u32,
// }

// pub struct Store {
//     gpu: Core,
//     storage: Grc<Buffer>,
//     allocations: Leaf<Vec<Grc<Allocation>>>,
// }

// impl Store {
//     pub fn new(gpu: Core) -> Self {
//         let storage = gpu.buffer(100000).storage().unwrap();
//         Self {
//             gpu,
//             storage,
//             allocations: Leaf::default(),
//         }
//     }
//     pub fn allocate_storage(&self, size: u32) -> Result<Grc<Allocation>> {
//         self.allocations.write_passive(|allocations| {
//             for alloc in allocations {

//             }
//         })?;
//         Err(anyhow!("failed to allocate gpu memory"))?
//     }
// }