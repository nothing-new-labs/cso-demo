use crate::statistics::{ColumnStat, ColumnStatSet, Statistics};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct MdId(u64);

pub trait RelationMd {
    fn output_row_count(&self) -> usize;
    fn column_md_ids(&self) -> &[MdId];
}

pub trait ColumnMd {
    fn column_stat(&self) -> ColumnStat;
}

#[allow(dead_code)]
pub enum Metadata {
    RelationMd(Rc<dyn RelationMd>),
    ColumnMd(Rc<dyn ColumnMd>),
}

pub trait MdProvider {
    fn retrieve_metadata(&self, md_id: &MdId) -> Rc<Metadata>;
}

pub type MdCache = HashMap<MdId, Rc<Metadata>>;

pub struct MdAccessor {
    md_cache: RefCell<MdCache>,
    md_provider: Rc<dyn MdProvider>,
}

impl MdAccessor {
    /// Construct a statistics object for the columns of the given relation
    pub fn derive_stats(&self, md_id: &MdId) -> Statistics {
        let relation_md = self.retrieve_relation_metadata(md_id);
        let output_row_count = relation_md.output_row_count();

        let column_md_ids = relation_md.column_md_ids();
        let column_stats = self.derive_column_stats(column_md_ids);

        Statistics::new(output_row_count, column_stats)
    }

    fn derive_column_stats(&self, column_md_ids: &[MdId]) -> ColumnStatSet {
        let mut column_stats = ColumnStatSet::with_capacity(column_md_ids.len());
        for (index, md_id) in column_md_ids.iter().enumerate() {
            let column_md = self.retrieve_column_metadata(md_id);
            let stat = column_md.column_stat();
            // todo
            column_stats.insert(index, stat);
        }

        column_stats
    }

    fn retrieve_relation_metadata(&self, md_id: &MdId) -> Rc<dyn RelationMd> {
        let md = self.retrieve_metadata(md_id);
        match md.as_ref() {
            Metadata::RelationMd(ret) => ret.clone(),
            Metadata::ColumnMd(_) => unreachable!("expected RelationMD"),
        }
    }

    fn retrieve_column_metadata(&self, md_id: &MdId) -> Rc<dyn ColumnMd> {
        let md = self.retrieve_metadata(md_id);
        match md.as_ref() {
            Metadata::RelationMd(_) => unreachable!("expected ColumnMD"),
            Metadata::ColumnMd(ret) => ret.clone(),
        }
    }

    fn retrieve_metadata(&self, md_id: &MdId) -> Rc<Metadata> {
        let mut md_cache = self.md_cache.borrow_mut();
        match md_cache.get(md_id) {
            Some(md) => md.clone(),
            None => {
                let md = self.md_provider.retrieve_metadata(md_id);
                md_cache.insert(*md_id, md.clone());
                md
            }
        }
    }
}
