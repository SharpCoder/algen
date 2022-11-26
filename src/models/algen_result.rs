use super::node::Node;

#[derive(Clone, Copy)]
pub struct AlgenResult<OutputData, Solution> {
    pub score: f32,
    pub output: Option<OutputData>,
    pub node: Option<Node<Solution>>,
}
