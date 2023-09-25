use crate::metadata::md_provider::MdProvider;
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

    pub fn retrieve_metadata(&self, md_id: &Box<dyn MdId>) -> Box<dyn Metadata> {
        let mut md_cache = self.md_cache.borrow_mut();
        match md_cache.get(md_id) {
            Some(md) => md.clone(),
            None => {
                let md = self.md_provider.retrieve_metadata(md_id);
                md_cache.insert(md_id.clone(), md.clone());
                md
            }
        }
    }
}
