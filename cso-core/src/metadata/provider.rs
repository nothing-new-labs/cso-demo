use crate::metadata::{MdCache, Metadata};
use crate::OptimizerType;

pub trait MdProvider<T: OptimizerType> {
    fn retrieve_metadata(&self, md_id: &T::MdId) -> Option<Box<dyn Metadata>>;
}

pub struct CachedMdProvider<T: OptimizerType> {
    md_cache: MdCache<T>,
}

impl<T: OptimizerType> CachedMdProvider<T> {
    pub fn new(md_cache: MdCache<T>) -> Self {
        Self { md_cache }
    }
}

impl<T: OptimizerType> MdProvider<T> for CachedMdProvider<T> {
    fn retrieve_metadata(&self, md_id: &T::MdId) -> Option<Box<dyn Metadata>> {
        match self.md_cache.get(md_id) {
            Some(md) => Some(md.clone()),
            None => None,
        }
    }
}
