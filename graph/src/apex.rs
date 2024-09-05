use super::*;

#[derive(Clone, Debug)]
pub enum Apex {
    String(Hub<String>),
}