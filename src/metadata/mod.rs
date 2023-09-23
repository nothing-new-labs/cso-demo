pub mod md_accessor;
pub mod md_provider;
pub mod statistics;

use crate::any::AsAny;
use dyn_clonable::clonable;
use serde::{Deserialize, Serialize};
use serde_json_any_key::any_key_map;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

#[typetag::serde(tag = "type")]
#[clonable]
pub trait Metadata: AsAny + Clone + Debug {}

impl dyn Metadata {
    #[inline]
    pub fn downcast_ref<T: Metadata>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[typetag::serde(tag = "type")]
#[clonable]
pub trait MdId: AsAny + Clone + Display + Debug {
    fn equal(&self, other: &dyn MdId) -> bool;
    fn hash(&self);
}

impl PartialEq<Self> for dyn MdId {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Hash for dyn MdId {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        self.hash()
    }
}

impl Eq for dyn MdId {}

impl dyn MdId {
    #[inline]
    pub fn downcast_ref<T: MdId>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

#[typetag::serde]
impl MdId for u64 {
    fn equal(&self, other: &dyn MdId) -> bool {
        let other = other.downcast_ref::<u64>().unwrap();
        self.eq(other)
    }

    fn hash(&self) {}
}

#[derive(Serialize, Deserialize)]
pub struct MdCache {
    #[serde(with = "any_key_map")]
    cache: HashMap<Box<dyn MdId>, Box<dyn Metadata>>,
}

impl MdCache {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn get(&self, key: &Box<dyn MdId>) -> Option<&Box<dyn Metadata>> {
        self.cache.get(key)
    }

    pub fn insert(&mut self, key: Box<dyn MdId>, val: Box<dyn Metadata>) -> Option<Box<dyn Metadata>> {
        self.cache.insert(key, val)
    }
}
