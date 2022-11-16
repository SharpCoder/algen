/// A node is a wrapper that contains one specific
/// solution for an experiment that was created
/// during a genetic run.
#[derive(Copy, Clone)]
pub struct Node<Solution> {
    pub id: usize,
    pub solution: Solution,
    pub score: f32,
}
