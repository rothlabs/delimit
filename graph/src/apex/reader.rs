use super::*;

pub struct Reader<'a> {
    apex: &'a Apex
}

// impl<'a> Reader<'a> {
//     pub fn string<T, F: FnOnce(&String) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
//         let wow = self.read(|tray| match tray {
//             Ok(Tray::String(value)) => Ok(read(value)),
//             _ => Ok(read(&"".into())),
//         });
//         wow.or_else(Ok(read(&"".into())))
//     }
// }