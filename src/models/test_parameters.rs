pub struct TestParameters {
    pub generations: usize,
    pub population: usize,
    pub elitism_factor: f32,
    pub crossover_factor: f32,
    pub mutation_factor: f32,
    pub purges: usize,
    pub tournament_size: usize,
}
