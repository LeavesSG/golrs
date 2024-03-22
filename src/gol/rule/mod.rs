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
            tiered_adj_factors: vec![1.0, 1.0, 1.0],
            alive_threshold: (0.20, 0.4),
            dead_threshold: (0.35, 0.4),
        }
    }
}

impl GolRule {
    pub fn normalized_tier_factors(&self, tier: usize) -> Vec<f64> {
        let using_tier = &self.tiered_adj_factors[0..tier];
        let sum: f64 = using_tier.iter().sum();
        using_tier.iter().map(|f| f / sum).collect()
    }
}
