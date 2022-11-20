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

mod math;
pub mod models;
mod telemetry;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{math::tournament_selection, models::node::Node};
use models::{algorithm::*, analyzer::Analyzer, test_parameters::TestParameters};
use rayon::prelude::*;
use telemetry::IterationTelemetry;

#[cfg(feature = "telemetry")]
use opentelemetry::{
    global,
    trace::{Span, Tracer},
    KeyValue,
};

fn time() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
}

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
    let run_id = time();
    let mut population = Vec::new();
    let mut next_population = Vec::new();

    for _ in 0..params.population {
        population.push(algo.allocate_node(&params));
    }

    // Iterate over each generation
    for generation in 0..params.generations {
        let generation_start = time();
        let mut best_score = 0.0;
        let mut best_output = None;

        // Compute the score for each node, in parallel
        let compute_start = time();
        population.par_iter_mut().for_each(|node| {
            let output = algo.output(node, &input_data, params);
            let score = analyzer.evaluate(&input_data, output.clone(), &params);
            node.score = score;
        });
        let compute_end = time();

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
        let combine_start = time();
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

        let combine_end = time();

        for child in children {
            next_population.push(child);
        }

        // Now promote next_pop into real pop
        population.clear();
        for node in next_population.clone() {
            population.push(node.clone());
        }
        next_population.clear();
        let generation_end = time();

        // Log telemetry. NOTE: This is only used for real if the telemetry featuer is enabled.
        log_telemetry(IterationTelemetry {
            run_id: run_id,
            generation: generation,
            generation_size: population.len(),
            total_compute_time_ms: (compute_end - compute_start),
            total_recombination_time_ms: (combine_end - combine_start),
            total_generation_time: (generation_end - generation_start),
            best_score: best_score,
            best_solution: best_output.clone(),
        });

        // Invoke the callback if present
        match on_generation_complete {
            None => {}
            Some(func) => match best_output {
                None => {}
                Some(output) => {
                    if func(output) {
                        return;
                    }
                }
            },
        }
    }
}

#[cfg(feature = "telemetry")]
fn log_telemetry<T>(telemetry: IterationTelemetry<T>) {
    let tracer = global::tracer("algen");
    let mut span = tracer.start("iteration");
    span.add_event_with_timestamp("generation", SystemTime::now(), Vec::new());
    span.set_attributes(vec![
        KeyValue::new("run_id", format!("{}", telemetry.run_id)),
        KeyValue::new("generation", telemetry.generation as i64),
        KeyValue::new("generation_size", telemetry.generation_size as i64),
        KeyValue::new(
            "total_compute_time_ms",
            telemetry.total_compute_time_ms as i64,
        ),
        KeyValue::new(
            "total_recombination_time_ms",
            telemetry.total_recombination_time_ms as i64,
        ),
        KeyValue::new(
            "total_generation_time",
            telemetry.total_generation_time as i64,
        ),
        KeyValue::new("best_score", telemetry.best_score as f64),
    ]);

    span.end();
}

#[cfg(not(feature = "telemetry"))]
fn log_telemetry<T>(telemetry: IterationTelemetry<T>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
