pub use plain::List;
// pub use html::Element

use serde::{Deserialize, Serialize};
use config::*;

pub mod html;
pub mod plain;

mod part;
mod config;

pub const PLAIN: usize = 1;
