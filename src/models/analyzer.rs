use super::test_parameters::TestParameters;

/// The Analyzer Trait is responsible for taking a set of test data,
/// the output of an algorithm, and then returning the score of how
/// well the algorithm did.
pub trait Analyzer<InputData, OutputData, FeatureFlags> {
    fn evaluate(&self, attempt: &OutputData, params: &TestParameters<FeatureFlags>) -> f32;
}
