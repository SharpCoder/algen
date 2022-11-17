use super::{node::Node, test_parameters::TestParameters};

/// An algorithm is a structure that represents the problem you are trying
/// to solve. It has methods for generating a random solution and evaluating
/// the solution in order to produce an output. Furthermore, it should know
/// how to recombine two solutions to produce the next generation.
pub trait Algorithm<InputData: Send + Sync, OutputData: Send + Sync, Solution: Clone + Send + Sync>
{
    /// A method which can take a test case and a Solution (effectively, the chromosome of the
    /// genetic algorithm) and return an output.
    ///
    /// An example of this might be:
    /// Given the input string "ELWWO"
    /// Given the Solution [3,-7,-11,-11,0]
    /// Apply the Solution (character shifts) to the original input
    ///
    /// Output -> "HELLO"
    ///
    /// This will later be scored with the analyzer.
    fn output(
        &self,
        node: &mut Node<Solution>,
        input: &InputData,
        params: &TestParameters,
    ) -> OutputData;

    /// This method should allocate a randomized Node<Solution>.
    fn allocate_node(&self, params: &TestParameters) -> Node<Solution>;

    /// Given two Node<Solution>, generate an offsprint using whatever
    /// genetic algorithm techniques you like. At a minimum, it should
    /// include:
    ///
    /// - Crossover
    /// - Mutation
    fn combine_node(
        &self,
        left: Node<Solution>,
        right: Node<Solution>,
        params: &TestParameters,
    ) -> Node<Solution>;
}
