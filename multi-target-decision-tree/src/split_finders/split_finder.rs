use common::{datasets::MultiTargetDataSet, results::BestSplitResult};
mod split_finder_variance;

pub enum SplitMetric {
    Variance,
}

pub struct SplitFinder {
    split_metric: SplitMetric,
    pub find_best_split: fn(&MultiTargetDataSet, u32) -> BestSplitResult,
}

impl SplitFinder {
    pub fn new(metric: SplitMetric) -> Self {
        Self {
            split_metric: metric,
            find_best_split: split_finder_variance::find_best_split,
        }
    }
}