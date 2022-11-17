pub mod models;
mod util;

use std::{
    borrow::BorrowMut,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{models::node::Node, util::tournament_selection};
use models::{algorithm::*, analyzer::Analyzer, test_parameters::TestParameters};
use rayon::prelude::*;

fn time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

pub fn run_genetic_test<
    InputData: Send + Sync,
    OutputData: Clone + Send + Sync,
    Solution: Clone + Send + Sync,
>(
    params: &TestParameters,
    input_data: InputData,
    algo: impl Algorithm<InputData, OutputData, Solution> + Sync,
    analyzer: impl Analyzer<InputData, OutputData> + Sync,

    on_generation_complete: Option<fn(OutputData)>,
) {
    // Generate the initial population
    let mut population = Vec::new();
    let mut next_population = Vec::new();

    for _ in 0..params.population {
        population.push(algo.allocate_node(&params));
    }

    // Iterate over each generation
    for generation in 0..params.generations {
        let start_time = time();
        let mut best_score = 0.0;
        let mut best_output = None;

        // Compute the score for each node, in parallel
        population.par_iter_mut().for_each(|node| {
            let output = algo.output(node, &input_data, params);
            let score = analyzer.evaluate(&input_data, output.clone(), &params);
            node.score = score;
        });

        population.iter_mut().for_each(|node| {
            if node.score > best_score {
                best_score = node.score;
                best_output = Some(algo.output(node, &input_data, params));
            }
        });

        // Retain the best and worst
        population.sort_by(|node_left, node_right| {
            node_right.score.partial_cmp(&node_left.score).unwrap()
        });

        // Take the creme of the crop, in both directions. And we multiply by 0.5
        // because each iteration takes 2 nodes.
        for i in 0..(params.elitism_factor * 0.5 * population.len() as f32) as usize {
            let bottom_idx = population.len() - i - 1;
            let top_node = population.get(i).unwrap().clone();
            let bottom_node = population.get(bottom_idx).unwrap().clone();
            next_population.push(top_node);
            next_population.push(bottom_node);
        }

        // NOTE!!! Consult Kozac on this logic
        // Now we need to fill up the population remaining with a population selection
        let children = population
            .par_iter()
            .map(|node| {
                let left = tournament_selection(population.as_slice(), params);
                let right = tournament_selection(population.as_slice(), params);

                if left.is_some() && right.is_some() {
                    return Some(algo.combine_node(left.unwrap(), right.unwrap(), params));
                } else {
                    return None;
                }
            })
            .take(population.len() - next_population.len())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Node<Solution>>>();

        for child in children {
            next_population.push(child);
        }

        // Now promote next_pop into real pop
        population.clear();
        for node in next_population.clone() {
            population.push(node.clone());
        }
        next_population.clear();

        // Invoke the callback if present
        match on_generation_complete {
            None => {}
            Some(func) => match best_output {
                None => {}
                Some(output) => {
                    func(output);
                }
            },
        }

        let end_time = time();
        let elapsed = end_time - start_time;
        println!("[{generation}] {elapsed}ms");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
