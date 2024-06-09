use std::{borrow::Cow, hash::Hash};

use serde::{Deserialize, Serialize};
use rand::distributions::{Alphanumeric, DistString};

#[derive(Default, Clone, Hash, PartialEq, Serialize, Deserialize)]
pub struct Id {
    cast: Cast,
    node: String,
    snap: String,
}

impl Id {
    pub fn new(cast: &'static str) -> Self {
        Id {
            cast: Cow::Borrowed(cast),
            node: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
            snap: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
}

impl Eq for Id {}

type Cast = Cow<'static, str>;