use super::*;

pub struct View<'a> {
    pub apex: &'a Apex
}

impl<'a> View<'a> {
    pub fn string<T, F: FnOnce(&String) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
        self.apex.read(|tray| match tray {
            Tray::String(value) => read(value),
            _ => Err(wrong_tray("String", tray.clone()))?
        })
    }
    pub fn vu8<T, F: FnOnce(&Vec<u8>) -> GraphResult<T>>(&self, read: F) -> GraphResult<T> {
        self.apex.read(|tray| match tray {
            Tray::Vu8(value) => read(value),
            _ => Err(wrong_tray("Vec<u8>", tray.clone()))?
        })
    }
}