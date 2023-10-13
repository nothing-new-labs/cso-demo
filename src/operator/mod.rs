use crate::Demo;

pub mod logical_filter;
pub mod logical_project;
pub mod logical_scan;
pub mod physical_filter;
pub mod physical_project;
pub mod physical_scan;
pub mod physical_sort;

pub type PhysicalOperator = dyn cso_core::operator::PhysicalOperator<OptimizerType = Demo>;
pub type LogicalOperator = dyn cso_core::operator::LogicalOperator<OptimizerType = Demo>;

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum OperatorId {
    LogicalScan,
    LogicalFilter,
    LogicalProject,

    PhysicalScan,
    PhysicalFilter,
    PhysicalProject,
    PhysicalSort,
}

impl cso_core::operator::OperatorId for OperatorId {}
