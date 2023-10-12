use crate::metadata::{MdCache, MdId, Metadata};

pub trait MdProvider {
    fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Option<Box<dyn Metadata>>;
}

pub struct CachedMdProvider {
    md_cache: MdCache,
}

impl CachedMdProvider {
    pub fn new(md_cache: MdCache) -> Self {
        Self { md_cache }
    }
}

impl MdProvider for CachedMdProvider {
    fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Option<Box<dyn Metadata>> {
        match self.md_cache.get(md_id.as_ref()) {
            Some(md) => Some(md.clone()),
            None => None,
        }
    }
}
