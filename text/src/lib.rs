pub use plain::List;

#[allow(unused_imports)]
use config::*;
use serde::{Deserialize, Serialize};

pub mod html;
pub mod plain;

mod config;
mod part;

pub const PLAIN: usize = 1;
