use super::*;

impl From<Tray> for Hub {
    fn from(value: Tray) -> Self {
        Hub::Tray(value)
    }
}

impl From<Leaf> for Hub {
    fn from(leaf: Leaf) -> Self {
        Hub::Leaf(leaf)
    }
}

impl From<Ploy> for Hub {
    fn from(ploy: Ploy) -> Self {
        Hub::Ploy(ploy)
    }
}

impl From<&Leaf> for Hub {
    fn from(value: &Leaf) -> Self {
        Hub::Leaf(value.clone())
    }
}

impl From<&Hub> for Hub {
    fn from(value: &Hub) -> Self {
        value.clone()
    }
}

impl From<&str> for Hub {
    fn from(value: &str) -> Self {
        Hub::Tray(Tray::String(value.to_owned()))
    }
}

impl From<String> for Hub {
    fn from(value: String) -> Self {
        Hub::Tray(Tray::String(value))
    }
}

impl From<u32> for Hub {
    fn from(value: u32) -> Self {
        Hub::Tray(Tray::U32(value))
    }
}

impl From<i32> for Hub {
    fn from(value: i32) -> Self {
        Hub::Tray(Tray::I32(value))
    }
}

impl From<Vec<u8>> for Hub {
    fn from(value: Vec<u8>) -> Self {
        Hub::Leaf(Leaf::new(Tray::Vu8(value)))
    }
}

impl From<Vec<u16>> for Hub {
    fn from(value: Vec<u16>) -> Self {
        Hub::Leaf(Leaf::new(Tray::Vu16(value)))
    }
}

impl From<Vec<f32>> for Hub {
    fn from(value: Vec<f32>) -> Self {
        Hub::Leaf(Leaf::new(Tray::Vf32(value)))
    }
}
