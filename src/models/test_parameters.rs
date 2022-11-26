/// This is a set of common genetic algorithm parameters that
/// are often used for testing purposes.
pub struct TestParameters<FeatureFlags> {
    /// The total amount of generations to produce in the test
    pub generations: usize,
    /// How many solutions will be created per generation
    pub population: usize,
    /// A number between 0 - 1 which indicates the percentage of results
    /// will be retained because they are the best (or worst) solutions.
    pub elitism_factor: f32,
    /// A number between 0 - 1 which indicates the probability of
    /// crossover favoring one solution over the other.
    pub crossover_factor: f32,
    /// A number between 0 - 1 which indicates the probability of
    /// mutation occuring for a specific bit.
    pub mutation_factor: f32,
    /// How many solutions will be included in the tournament selection
    /// event, per tournament.
    pub tournament_size: usize,
    /// A bucket of strings that you can use however you like.
    pub feature_flag: FeatureFlags,
}
