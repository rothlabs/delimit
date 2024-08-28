use super::*;

pub struct View<'a> {
    pub apex: &'a Apex
}

impl<'a> View<'a> {
    pub fn string<T, F: FnOnce(&String) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
        self.apex.read(|tray| match tray {
            Tray::String(string) => read(string),
            _ => Err(wrong_tray("String", tray.clone()))?
        })
    }
}