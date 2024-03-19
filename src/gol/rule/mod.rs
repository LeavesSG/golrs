pub struct GolRule {
    // The factors of a series of adjacent cells.
    pub tiered_adj_factors: Vec<f64>,

    // The threshold to determine a live block to be "died" or "alive".
    // First element in the tuple as the lower bound, second element as the upper bound.
    pub alive_threshold: (f64, f64),

    // The threshold to determine a dead block to be "died" or "alive".
    // First element in the tuple as the lower bound, second element as the upper bound.
    pub dead_threshold: (f64, f64),
}

impl Default for GolRule {
    fn default() -> Self {
        Self {
            tiered_adj_factors: vec![1., 1., 1.],
            alive_threshold: (0.25, 0.5),
            dead_threshold: (0.375, 0.5),
        }
    }
}

impl GolRule {
    pub fn normalized_tier_factors(&self) -> Vec<f64> {
        let sum: f64 = self.tiered_adj_factors.iter().sum();
        self.tiered_adj_factors.iter().map(|f| f / sum).collect()
    }
}
