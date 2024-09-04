use super::*;

impl From<Tray> for Apex {
    fn from(value: Tray) -> Self {
        Apex::Tray(value)
    }
}

impl From<Leaf> for Apex {
    fn from(leaf: Leaf) -> Self {
        Apex::Leaf(leaf)
    }
}

impl From<Ploy> for Apex {
    fn from(ploy: Ploy) -> Self {
        // TODO: find way to get rank during creation of ploy or node?
        Apex::Ploy(ploy.ranked())
    }
}

impl From<&Leaf> for Apex {
    fn from(value: &Leaf) -> Self {
        Apex::Leaf(value.clone())
    }
}

impl From<&Apex> for Apex {
    fn from(value: &Apex) -> Self {
        value.clone()
    }
}

impl From<&str> for Apex {
    fn from(value: &str) -> Self {
        Apex::Tray(Tray::String(value.to_owned()))
    }
}

impl From<String> for Apex {
    fn from(value: String) -> Self {
        Apex::Tray(Tray::String(value))
    }
}

impl From<u32> for Apex {
    fn from(value: u32) -> Self {
        Apex::Tray(Tray::U32(value))
    }
}

impl From<i32> for Apex {
    fn from(value: i32) -> Self {
        Apex::Tray(Tray::I32(value))
    }
}

impl From<Vec<u8>> for Apex {
    fn from(value: Vec<u8>) -> Self {
        Apex::Leaf(Leaf::new(Tray::Vu8(value)))
    }
}

impl From<Vec<u16>> for Apex {
    fn from(value: Vec<u16>) -> Self {
        Apex::Leaf(Leaf::new(Tray::Vu16(value)))
    }
}

impl From<Vec<f32>> for Apex {
    fn from(value: Vec<f32>) -> Self {
        Apex::Leaf(Leaf::new(Tray::Vf32(value)))
    }
}

// // TODO: find way to not query the apex to get rank!
// let rank = match ploy.main() {
//     Ok(apex) => match apex.rank() {
//         Some(rank) => rank + 1,
//         None => 1,
//     },
//     _ => 0,
// };
