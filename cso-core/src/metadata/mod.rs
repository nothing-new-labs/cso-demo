mod accessor;
mod provider;
mod statistics;

pub use self::accessor::MdAccessor;
pub use self::provider::{CachedMdProvider, MdProvider};
pub use self::statistics::Stats;

use crate::any::AsAny;
use crate::OptimizerType;
use dyn_clonable::clonable;
use serde::{Deserialize, Serialize};
use serde_json_any_key::any_key_map;
use std::collections::HashMap;
use std::fmt::Debug;

#[typetag::serde(tag = "type")]
#[clonable]
pub trait Metadata: AsAny + Clone + Debug {}

impl dyn Metadata {
    #[inline]
    pub fn downcast_ref<T: Metadata>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[derive(Serialize, Deserialize)]
pub struct MdCache<T: OptimizerType> {
    #[serde(with = "any_key_map")]
    cache: HashMap<T::MdId, Box<dyn Metadata>>,
}

impl<T: OptimizerType> MdCache<T> {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn get(&self, key: &T::MdId) -> Option<&Box<dyn Metadata>> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: T::MdId, val: Box<dyn Metadata>) -> Option<Box<dyn Metadata>> {
        self.cache.insert(key, val)
    }
}
