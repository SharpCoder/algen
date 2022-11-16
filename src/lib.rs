use models::{algorithm::*, analyzer::Analyzer, node::Node, test_parameters::TestParameters};

use crate::util::tournament_selection;

pub mod models;
mod util;

pub fn run<InputData, Solution: Clone>(
    params: &TestParameters,
    input_data: InputData,
    algo: impl Algorithm<InputData, Solution>,
    analyzer: impl Analyzer<InputData, Solution>,
) {
    // Generate the initial population
    let mut population = Vec::new();
    let mut next_population = Vec::new();

    for _ in 0..params.population {
        population.push(algo.allocate_node(&params));
    }

    // Iterate over each generation
    for generation in 0..params.generations {
        println!("Processing generation {generation}");

        // Compute the score for each node
        for idx in 0..population.len() {
            let mut node = population.get_mut(idx);
            match node {
                Some(mutable_node) => {
                    mutable_node.score = analyzer.evaluate(&input_data, mutable_node, &params);
                }
                None => (),
            }
        }

        // Retain the best and worst
        population.sort_by(|node_left, node_right| {
            node_right.score.partial_cmp(&node_left.score).unwrap()
        });

        // Take the creme of the crop, in both directions. And we multiply by 0.5
        // because each iteration takes 2 nodes.
        for i in 0..(params.elitism_factor * 0.5 * population.len() as f32) as usize {
            let bottom_idx = population.len() - i;
            let top_node = population.get_mut(i).unwrap().clone();
            let bottom_node = population.get_mut(bottom_idx).unwrap().clone();
            next_population.push(top_node);
            next_population.push(bottom_node);
        }

        // NOTE!!! Consult Kozac on this logic
        // Now we need to fill up the population remaining with a population selection
        while next_population.len() != population.len() {
            let left = tournament_selection(population.as_slice(), params);
            let right = tournament_selection(population.as_slice(), params);

            if left.is_some() && right.is_some() {
                next_population.push(algo.combine_node(left.unwrap(), right.unwrap(), params));
            }
        }

        // Now promote next_pop into real pop
        population.clear();
        for node in next_population.clone() {
            population.push(node.clone());
        }
        next_population.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
