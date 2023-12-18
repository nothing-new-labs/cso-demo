mod exploration;
mod implementation;

use crate::rule::exploration::filter_2_index_scan::Filter2IndexScan;
use crate::rule::implementation::filter::FilterImplementation;
use crate::rule::implementation::index_scan::IndexScanImplementation;
use crate::rule::implementation::project::ProjectImplementation;
use crate::rule::implementation::scan::ScanImplementation;
use crate::Demo;
use cso_core::rule::RuleSet;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum RuleId {
    ScanImplementation = 1,
    FilterImplementation = 2,
    ProjectImplementation = 3,
    IndexScanImplementation = 4,
    Filter2IndexScan = 5,
}

impl cso_core::rule::RuleId for RuleId {
    fn as_usize(self) -> usize {
        self as usize
    }
}

pub fn create_rule_set() -> RuleSet<Demo> {
    let mut rule_set = RuleSet::new();
    rule_set.set_implement_rules(vec![
        Rc::new(ScanImplementation::new()),
        Rc::new(FilterImplementation::new()),
        Rc::new(ProjectImplementation::new()),
        Rc::new(IndexScanImplementation::new()),
    ]);
    rule_set.set_transform_rules(vec![Rc::new(Filter2IndexScan::new())]);
    rule_set
}
