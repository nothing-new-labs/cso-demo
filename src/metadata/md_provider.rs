use crate::metadata::{MdCache, MdId, Metadata};

pub trait MdProvider {
    fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Option<Box<dyn Metadata>>;
}

pub struct TempMdProvider {
    md_cache: MdCache,
}

impl TempMdProvider {
    pub fn new(md_cache: MdCache) -> Self {
        Self { md_cache }
    }
}

impl MdProvider for TempMdProvider {
    fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Option<Box<dyn Metadata>> {
        match self.md_cache.get(md_id) {
            Some(md) => Some(md.clone()),
            None => None,
        }
    }
}
