use super::*;

pub mod draw {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Direct {
        pub vertices: Range<u32>,
        pub instances: Range<u32>,
    }
}
