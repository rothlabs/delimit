#[cfg(test)]
mod test;

pub mod text;
//pub mod html;

pub use text::list::List;
pub use text::{Text, TextList};

// pub use text::{
//     Text,
//     // Task,
//     // Load,
// };

// pub use text::unit::list;
// pub use html::unit::doc;

// #[cfg(test)]
// mod tests {
//     use super::text::*;

//     #[test]
//     fn list_empty() {
//         let list = list();
//         assert_eq!(list.string().0.as_str(), "");
//     }
// }
