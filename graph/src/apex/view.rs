use super::*;

pub struct View<'a> {
    pub apex: &'a Apex,
}

impl<'a> View<'a> {
    /// String reader
    pub fn string<T, F: FnOnce(Result<&String>) -> Result<T>>(&self, read: F) -> Result<T> {
        self.apex.read(|tray| {
            let tray = tray?;
            match tray {
                Tray::String(value) => read(Ok(value)),
                _ => Err(wrong_tray("String", tray))?,
            }
        })
    }

    /// Vec<u8> reader
    pub fn vec_u8<T, F: FnOnce(Result<&Vec<u8>>) -> Result<T>>(&self, read: F) -> Result<T> {
        self.apex.read(|tray| {
            let tray = tray?;
            match tray {
                Tray::Vu8(value) => read(Ok(value)),
                _ => Err(wrong_tray("Vec<u8>", tray))?,
            }
        })
    }
}
