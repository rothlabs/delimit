use std::sync::Arc;

use serde::{Serialize, Serializer};

pub struct Node(pub Arc<dyn crate::Nodish>);


// impl Serialize for Node {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
        
//         serializer.serialize_str(self.0.id())
//     }
// }

