use crate::metadata::provider::MdProvider;
use crate::metadata::{MdCache, Metadata};
use crate::OptimizerType;
use std::cell::RefCell;
use std::rc::Rc;

pub struct MdAccessor<T: OptimizerType> {
    md_cache: RefCell<MdCache<T>>,
    md_provider: Rc<dyn MdProvider<OptimizerType = T>>,
}

impl<T: OptimizerType> MdAccessor<T> {
    pub fn new(md_provider: Rc<dyn MdProvider<OptimizerType = T>>) -> Self {
        Self {
            md_cache: RefCell::new(MdCache::new()),
            md_provider,
        }
    }

    pub fn retrieve_metadata(&self, md_id: &T::MdId) -> Option<Box<dyn Metadata>> {
        let mut md_cache = self.md_cache.borrow_mut();
        match md_cache.get(md_id) {
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
