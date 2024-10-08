pub use plain::List;

use serde::{Deserialize, Serialize};

pub mod html;
pub mod plain;

pub const PLAIN: u16 = 1;

#[macro_use]
extern crate macro_rules_attribute;
