pub use plain::List;

use serde::{Deserialize, Serialize};

pub mod html;
pub mod plain;

pub const PLAIN: usize = 1;
