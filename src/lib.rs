//! Algen is a genetic algorithm runner. It provides a common set of traits
//! and models that can be implemented to construct a genetic algorithm.
//! Once these traits and models have been filled out, you can invoke the
//! `run_algorithm` method in this crate which will do the following:
//!
//! - Create an initial population
//! - Score each node
//! - Reserve the best and worst solutions
//! - Create the next generation through recombination and tournament selection
//! - Begin the cycle again
//!
//! This will happen until a winning condition is met, or until you have
//! exhausted all generations.
//!
//! Algen provides an abstraction on top of genetic algorithms. On its own, it
//! does not provide a working implementation. That's up to you! So here are the
//! traits you need to implement in order to use Algen:
//!
//! - **Algorithm** to define how input data is manipulated to solve a particular problem.
//! - **Analyzer** to score the result of the algorithm and produce a numeric value representing how well it did.
//!
//! In addition to these traits, you need to provide [TestParameters](https://docs.rs/algen/latest/algen/models/test_parameters/index.html) and
//! some kind of **Input Data** which is fed to your algorithm.
//!
//! See the example in the examples folder for more details.
//!
//! ```no_run
//!     run_algorithm(
//!         &parameters,
//!         test_data,
//!         algo,
//!         analyzer,
//!         Some(after_generation),
//!     );
//! ```
mod math;
pub mod models;

use crate::{
    math::tournament_selection, models::algorithm::*, models::analyzer::Analyzer,
    models::node::Node, models::test_parameters::TestParameters,
};
use rayon::prelude::*;

#[cfg(feature = "tracing")]
use tracing::{event, span, Level};

/// The primary algorithm runner. This method will accept the types:
/// - InputData: The shape of data which is passed to each solution.
/// - OutputData: The shape of data which a solution will output
/// - Solution: The chromosome which represents a solution
///
/// Additionally, it takes the following parameters:
/// - params: Test parameters that define the rules of the runner
/// - input_data: The actual data to pass to each solution
/// - algo: A struct which implements the Algorithm trait
/// - analyzer: A struct which implements the Analyzer trait
/// - on_generation_complete: A method which is run at the end of each
/// generation and, if it returns true, the test will be stopped.
pub fn run_algorithm<
    InputData: Send + Sync,
    OutputData: Clone + Send + Sync,
    Solution: Clone + Send + Sync,
>(
    params: &TestParameters,
    input_data: InputData,
    algo: impl Algorithm<InputData, OutputData, Solution> + Sync,
    analyzer: impl Analyzer<InputData, OutputData> + Sync,

    on_generation_complete: Option<fn(OutputData) -> bool>,
) {
    // Generate the initial population
    let mut population = Vec::new();
    let mut next_population = Vec::new();

    for _ in 0..params.population {
        population.push(algo.allocate_node(&params));
    }

    // Iterate over each generation
    for generation in 0..params.generations {
        #[cfg(feature = "tracing")]
        let generation_span = span!(Level::TRACE, "generation", generation = generation);
        #[cfg(feature = "tracing")]
        let generation_span_entered = generation_span.enter();

        let mut best_score = 0.0;
        let mut best_output = None;

        // Compute the score for each node, in parallel
        #[cfg(feature = "tracing")]
        let compute_span = span!(Level::TRACE, "compute");
        #[cfg(feature = "tracing")]
        let compute_span_entered = compute_span.enter();

        population.par_iter_mut().for_each(|node| {
            let output = algo.output(node, &input_data, params);
            let score = analyzer.evaluate(&input_data, output.clone(), &params);
            node.score = score;
        });

        #[cfg(feature = "tracing")]
        drop(compute_span_entered);

        // Find the best solution
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

        #[cfg(feature = "tracing")]
        let next_generation_span = span!(Level::TRACE, "recombination");
        #[cfg(feature = "tracing")]
        let next_generation_span_entered = next_generation_span.enter();

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
            .map(|_| {
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

        #[cfg(feature = "tracing")]
        drop(next_generation_span_entered);

        // Now promote next_pop into real pop
        population.clear();
        for node in next_population.clone() {
            population.push(node.clone());
        }
        next_population.clear();

        #[cfg(feature = "tracing")]
        event!(
            Level::INFO,
            msg = "Generation finished processing",
            generation = generation,
            score = best_score
        );

        // Invoke the callback if present
        match on_generation_complete {
            None => {}
            Some(func) => match best_output {
                None => {}
                Some(output) => {
                    if func(output) {
                        #[cfg(feature = "tracing")]
                        event!(
                            Level::INFO,
                            msg = "Winning condition met",
                            best_score = best_score
                        );
                        return;
                    }
                }
            },
        }

        #[cfg(feature = "tracing")]
        drop(generation_span_entered);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
