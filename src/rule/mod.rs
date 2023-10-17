mod implementation;

use crate::rule::implementation::filter::FilterImplementation;
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
    ]);
    rule_set
}
