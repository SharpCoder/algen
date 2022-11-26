use crate::models::{node::Node, test_parameters::TestParameters};
use rand::prelude::*;

/// A basic implementation of tournament selection.
pub fn tournament_selection<Solution: Clone, FeatureFlags>(
    nodes: &[Node<Solution>],
    params: &TestParameters<FeatureFlags>,
) -> Option<Node<Solution>> {
    let mut rng = rand::thread_rng();
    let mut best_node: Option<Node<Solution>> = None;
    let mut best_score = f32::MIN;

    for _ in 0..params.tournament_size {
        let idx = rng.gen_range(0..nodes.len());
        match nodes.get(idx) {
            Some(node) => {
                if node.score > best_score {
                    best_node = Some(node.clone());
                    best_score = node.score;
                }
            }
            None => (),
        }
    }

    return best_node;
}
