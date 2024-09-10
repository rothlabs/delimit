use super::*;
use std::ops::Deref;

/// `Vec<f32>` for Graph. Includes required Hash implementation.
#[derive(Default, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Vf32(pub Vec<f32>);

impl Hash for Vf32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0
            .iter()
            .map(|x| x.to_bits())
            .collect::<Vec<u32>>()
            .hash(state)
    }
}

impl Deref for Vf32 {
    type Target = Vec<f32>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl  {
    
// }
