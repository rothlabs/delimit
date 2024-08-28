use super::*;

pub struct Sure<'a> {
    pub apex: &'a Apex
}

impl<'a> Sure<'a> {
    pub fn string<T, F: FnOnce(&String) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
        self.apex.read(|tray| match tray {
            Tray::String(string) => read(string),
            _ => read(&"".into())
        })
    }
}