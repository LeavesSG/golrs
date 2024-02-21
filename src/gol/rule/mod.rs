pub struct GolRule {
    // The threshold to determine a block to be "died" or "alive".
    // First element in the tuple as the upper bound, second element as the upper bound.
    alive_threshold: (f64, f64),

    // Factor multiplied as adjacent blocks counted as "alive" when shared the same vertex.
    shared_vtx_factor: f64,

    // Factor multiplied as adjacent blocks counted as "alive" when shared the same edge.
    shared_edge_factor: f64,

    // Factor multiplied as adjacent blocks counted as "alive" when shared the same face.
    shared_face_factor: f64,
}

impl Default for GolRule {
    fn default() -> Self {
        Self {
            alive_threshold: (2.0, 3.0),
            shared_vtx_factor: 1.0,
            shared_edge_factor: 1.0,
            shared_face_factor: 1.0,
        }
    }
}
