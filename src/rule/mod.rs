mod implementation;

use crate::rule::implementation::filter::FilterImplementation;
use crate::rule::implementation::project::ProjectImplementation;
use crate::rule::implementation::scan::ScanImplementation;
use cso_core::rule::RuleSet;
use std::rc::Rc;

pub fn create_rule_set() -> RuleSet {
    let mut rule_set = RuleSet::new();
    rule_set.register_implement_rules(vec![
        Rc::new(ScanImplementation::new()),
        Rc::new(FilterImplementation::new()),
        Rc::new(ProjectImplementation::new()),
    ]);
    rule_set
}
