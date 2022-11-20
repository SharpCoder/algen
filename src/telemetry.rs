#[derive(Copy, Clone)]
pub struct IterationTelemetry<OutputType> {
    /// Which generation this blob refers to
    pub generation: usize,
    /// How many solutions exist in the population
    pub generation_size: usize,
    /// How much time it took to evaluate all solutions
    pub total_compute_time_ms: u128,
    /// How much time it took to produce the next generation through recombination
    pub total_recombination_time_ms: u128,
    /// How much time it toook to process the entire generation
    pub total_generation_time: u128,
    /// The best score of this generation
    pub best_score: f32,
    /// The best solution of this generation
    pub best_solution: Option<OutputType>,
}
