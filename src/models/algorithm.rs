use super::{node::Node, test_parameters::TestParameters};

/// An algorithm is a structure that represents the problem you are trying
/// to solve. It has methods for generating a random solution and evaluating
/// the solution in order to produce an output. Furthermore, it should know
/// how to recombine two solutions to produce the next generation.
pub trait Algorithm<InputData, Solution: Clone> {
    fn output<ComputedOutput>(
        &self,
        node: Node<Solution>,
        input: InputData,
        params: &TestParameters,
    ) -> ComputedOutput;

    fn allocate_node(&self, params: &TestParameters) -> Node<Solution>;

    fn combine_node(
        &self,
        left: Node<Solution>,
        right: Node<Solution>,
        params: &TestParameters,
    ) -> Node<Solution>;
}
