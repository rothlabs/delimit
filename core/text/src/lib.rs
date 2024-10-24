pub use plain::List;

use serde::{Deserialize, Serialize};
use node_derive::{Adapt, Digest};

pub mod html;
pub mod plain;

pub const PLAIN: u16 = 1;

#[macro_use]
extern crate macro_rules_attribute;
