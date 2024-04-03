pub const COST_INIT_SCAN_FACTOR: f64 = 431.0; // scan initialization cost factor
pub const COST_TABLE_SCAN_COST_UNIT: f64 = 5.50e-07; // table scan cost per tuple
pub const COST_INDEX_FILTER_COST_UNIT: f64 = 1.65e-04; // index filtering cost unit
pub const COST_INDEX_SCAN_TUP_COST_UNIT: f64 = 3.66e-06; // index scan cost unit per tuple per width
pub const COST_INDEX_SCAN_TUP_RANDOM_FACTOR: f64 = 6.0; // index scan random IO factor
pub const COST_FILTER_COL_COST_UNIT: f64 = 3.29e-05; // filter column cost unit
pub const COST_TUP_DEFAULT_PROC_COST_UNIT: f64 = 1.0e-06; // cost for processing per tuple with unit width
pub const COST_SORT_TUP_WIDTH_COST_UNIT: f64 = 5.67e-06; // sorting cost per tuple with unit width
