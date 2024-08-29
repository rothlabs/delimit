use super::*;

pub struct View<'a> {
    pub apex: &'a Apex,
}

impl<'a> View<'a> {
    /// String reader
    pub fn string<T, F: FnOnce(GraphResult<&String>) -> GraphResult<T>>(
        &self,
        read: F,
    ) -> GraphResult<T> {
        self.apex.read(|tray| {
            let tray = tray?;
            match tray {
                Tray::String(value) => read(Ok(value)),
                _ => Err(wrong_tray("String", tray.clone()))?,
            }
        })
    }

    /// Vec<u8> reader
    pub fn vec_u8<T, F: FnOnce(GraphResult<&Vec<u8>>) -> GraphResult<T>>(
        &self,
        read: F,
    ) -> GraphResult<T> {
        self.apex.read(|tray| {
            let tray = tray?;
            match tray {
                Tray::Vu8(value) => read(Ok(value)),
                _ => Err(wrong_tray("Vec<u8>", tray.clone()))?,
            }
        })
    }
}
