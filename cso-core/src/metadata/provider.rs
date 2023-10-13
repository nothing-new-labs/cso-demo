use crate::metadata::{MdCache, Metadata};
use crate::OptimizerType;

pub trait MdProvider {
    type OptimizerType: OptimizerType;

    fn retrieve_metadata(&self, md_id: &<Self::OptimizerType as OptimizerType>::MdId) -> Option<Box<dyn Metadata>>;
}

pub struct CachedMdProvider<T: OptimizerType> {
    md_cache: MdCache<T>,
}

impl<T: OptimizerType> CachedMdProvider<T> {
    pub fn new(md_cache: MdCache<T>) -> Self {
        Self { md_cache }
    }
}

impl<T: OptimizerType> MdProvider for CachedMdProvider<T> {
    type OptimizerType = T;

    fn retrieve_metadata(&self, md_id: &T::MdId) -> Option<Box<dyn Metadata>> {
        match self.md_cache.get(md_id) {
            Some(md) => Some(md.clone()),
            None => None,
        }
    }
}
