use super::{node::Node, test_parameters::TestParameters};

pub trait Analyzer<InputData, OutputData> {
    fn evaluate(&self, input: &InputData, attempt: OutputData, params: &TestParameters) -> f32;
}
