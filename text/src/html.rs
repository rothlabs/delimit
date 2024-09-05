pub use attribute::*;
pub use default::*;
pub use element::*;
pub use tag::*;

use super::*;
use graph::*;

pub mod default;

mod attribute;
mod element;
mod tag;
#[cfg(test)]
mod tests;

// pub fn tags() -> Hub {
//     let mut bay = Bay::new();
//     for tag in TAGS {
//         bay.insert(tag, tag.leaf());
//     }
//     bay.hub()
// }

// pub fn attributes() -> Hub {
//     let mut bay = Bay::new();
//     for tag in ATTRIBUTES {
//         bay.insert(tag, tag.leaf());
//     }
//     bay.hub()
// }
