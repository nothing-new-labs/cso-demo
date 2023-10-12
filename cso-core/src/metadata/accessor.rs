use crate::metadata::provider::MdProvider;
use crate::metadata::{MdCache, MdId, Metadata};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MdAccessor {
    md_cache: RefCell<MdCache>,
    md_provider: Rc<dyn MdProvider>,
}

impl MdAccessor {
    pub fn new(md_provider: Rc<dyn MdProvider>) -> Self {
        Self {
            md_cache: RefCell::new(MdCache::new()),
            md_provider,
        }
    }

    pub fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Option<Box<dyn Metadata>> {
        let mut md_cache = self.md_cache.borrow_mut();
        match md_cache.get(md_id.as_ref()) {
            Some(md) => Some(md.clone()),
            None => match self.md_provider.retrieve_metadata(md_id) {
                Some(md) => {
                    md_cache.insert(md_id.clone(), md.clone());
                    Some(md)
                }
                None => None,
            },
        }
    }
}
