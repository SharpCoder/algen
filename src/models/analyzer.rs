use super::{node::Node, test_parameters::TestParameters};

pub trait Analyzer<InputData, Solution: Clone> {
    fn evaluate(&self, input: &InputData, node: &Node<Solution>, params: &TestParameters) -> f32;
}
