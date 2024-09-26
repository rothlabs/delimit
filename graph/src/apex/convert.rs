use super::*;

impl From<&Apex> for Apex {
    fn from(value: &Apex) -> Self {
        match value {
            Apex::Void(x) => Apex::Void(x.clone()),
            Apex::String(x) => Apex::String(x.clone()),
            Apex::U8(x) => Apex::U8(x.clone()),
            Apex::I32(x) => Apex::I32(x.clone()),
            Apex::Vu8(x) => Apex::Vu8(x.clone()),
            Apex::Vu16(x) => Apex::Vu16(x.clone()),
            Apex::Vf32(x) => Apex::Vf32(x.clone()),
            Apex::Vf64(x) => Apex::Vf64(x.clone()),
        }
    }
}

impl From<Hub<()>> for Apex {
    fn from(value: Hub<()>) -> Self {
        Apex::Void(value)
    }
}

impl From<&Hub<i32>> for Apex {
    fn from(value: &Hub<i32>) -> Self {
        Apex::I32(value.clone())
    }
}

impl From<&str> for Apex {
    fn from(value: &str) -> Self {
        Apex::String(value.into())
    }
}

impl From<Hub<String>> for Apex {
    fn from(value: Hub<String>) -> Self {
        Apex::String(value)
    }
}

impl From<Leaf<String>> for Apex {
    fn from(value: Leaf<String>) -> Self {
        Apex::String(value.into())
    }
}

impl From<i32> for Apex {
    fn from(value: i32) -> Self {
        Apex::I32(value.into())
    }
}

impl From<Vec<u8>> for Apex {
    fn from(value: Vec<u8>) -> Self {
        Apex::Vu8(value.into())
    }
}

impl From<Vec<u16>> for Apex {
    fn from(value: Vec<u16>) -> Self {
        Apex::Vu16(value.into())
    }
}

impl From<Vec<f32>> for Apex {
    fn from(value: Vec<f32>) -> Self {
        Apex::Vf32(value.into())
    }
}

impl From<Vec<f64>> for Apex {
    fn from(value: Vec<f64>) -> Self {
        Apex::Vf64(value.into())
    }
}
