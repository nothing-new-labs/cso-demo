pub struct Statistics {
    output_row_count: usize,
}

impl Statistics {
    pub const fn should_update(new_stats: &Statistics, old_stats: &Statistics) -> bool {
        new_stats.output_row_count < old_stats.output_row_count
    }
}
