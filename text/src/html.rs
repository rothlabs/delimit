pub use attribute::*;
pub use default::*;
pub use element::*;
pub use tag::*;

use super::*;
use graph::*;

mod attribute;
mod default;
mod element;
mod tag;
#[cfg(test)]
mod tests;

// pub fn tags() -> Apex {
//     let mut bay = Bay::new();
//     for tag in TAGS {
//         bay.insert(tag, tag.leaf());
//     }
//     bay.apex()
// }

// pub fn attributes() -> Apex {
//     let mut bay = Bay::new();
//     for tag in ATTRIBUTES {
//         bay.insert(tag, tag.leaf());
//     }
//     bay.apex()
// }
