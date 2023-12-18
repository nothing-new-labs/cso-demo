use crate::Demo;

pub mod logical_filter;
pub mod logical_index_scan;
pub mod logical_project;
pub mod logical_scan;
pub mod physical_filter;
pub mod physical_index_scan;
pub mod physical_project;
pub mod physical_scan;
pub mod physical_sort;

pub type PhysicalOperator = dyn cso_core::operator::PhysicalOperator<Demo>;
pub type LogicalOperator = dyn cso_core::operator::LogicalOperator<Demo>;

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum OperatorId {
    LogicalScan,
    LogicalFilter,
    LogicalProject,
    LogicalIndexScan,

    PhysicalScan,
    PhysicalIndexScan,
    PhysicalFilter,
    PhysicalProject,
    PhysicalSort,
}
