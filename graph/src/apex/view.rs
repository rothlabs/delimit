use super::*;

pub struct View<'a> {
    pub apex: &'a Apex,
}

impl<'a> View<'a> {
    /// String reader
    pub fn string<T, F: FnOnce(&String) -> T>(&self, read: F) -> Result<T> {
        self.apex.read(|tray| match tray {
            Tray::String(value) => Ok(read(value)),
            _ => Err(wrong_tray("String", tray))?,
        })?
    }

    /// Vec<u8> reader
    pub fn vec_u8<T, F: FnOnce(&Vec<u8>) -> T>(&self, read: F) -> Result<T> {
        self.apex.read(|tray| match tray {
            Tray::Vu8(value) => Ok(read(value)),
            _ => Err(wrong_tray("String", tray))?,
        })?
    }
}
