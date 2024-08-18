pub use attribute::*;
// pub use doc::*;
pub use element::Element;
pub use tag::*;

use super::*;
use graph::*;

mod attribute;
// mod doc;
mod element;
mod tag;
#[cfg(test)]
mod tests;

pub fn tags() -> Apex {
    let mut bay = Bay::new();
    for tag in TAGS {
        bay.insert(tag, tag.leaf());
    }
    bay.apex()
}

// pub fn attributes() -> Apex {
//     let mut bay = Bay::new();
//     for tag in ATTRIBUTES {
//         bay.insert(tag, tag.leaf());
//     }
//     bay.apex()
// }