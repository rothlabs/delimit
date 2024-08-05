pub use plain::List;
// pub use html::Element

use serde::{Deserialize, Serialize};

pub mod html;
pub mod plain;

mod part;

pub const PLAIN: usize = 1;
