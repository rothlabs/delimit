use thiserror::Error;
use super::*;

#[derive(Error, Debug)]
pub enum ApexError {
    #[error("not ploy")]
    NotPloy
}