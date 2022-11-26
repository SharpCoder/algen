use std::str::FromStr;

use algen::{
    models::{
        algorithm::Algorithm, analyzer::Analyzer, node::Node, test_parameters::TestParameters,
    },
    run_algorithm,
};
use rand::prelude::*;

type Unit = i16;
type InputType = [Unit; 13];
type OutputType = String;
type FeatureFlags = Vec<&'static str>;

#[derive(Clone)]
struct Solution {
    shifts: InputType,
}

struct GeneticAlgorithm {}
impl Algorithm<InputType, OutputType, Solution, FeatureFlags> for GeneticAlgorithm {
    fn allocate_node(
        &self,
        _input: &InputType,
        _params: &TestParameters<FeatureFlags>,
    ) -> Node<Solution> {
        let mut rng = rand::thread_rng();
        let mut solution = Solution { shifts: [0; 13] };

        for idx in 0..13 {
            solution.shifts[idx] = rng.gen_range(-128..128);
        }

        return Node {
            id: 0,
            score: f32::MIN,
            solution: solution,
        };
    }

    fn combine_node(
        &self,
        left: Node<Solution>,
        right: Node<Solution>,
        params: &TestParameters<FeatureFlags>,
    ) -> Node<Solution> {
        let mut rng = rand::thread_rng();
        let mut next_solution: InputType = [0; 13];

        for i in 0..13 {
            if rng.gen_bool(params.crossover_factor as f64) {
                next_solution[i] = left.solution.shifts[i];
            } else {
                next_solution[i] = right.solution.shifts[i];
            }

            if rng.gen_bool(params.mutation_factor as f64) {
                next_solution[i] = rng.gen_range(-128..128)
            }
        }

        return Node {
            id: 0,
            score: f32::MIN,
            solution: Solution {
                shifts: next_solution,
            },
        };
    }

    fn output(
        &self,
        node: &Node<Solution>,
        input: &InputType,
        _params: &TestParameters<FeatureFlags>,
    ) -> OutputType {
        let mut output: [u8; 13] = [0; 13];
        for i in 0..13 {
            let byte = input[i] + node.solution.shifts[i];
            if byte < 0 {
                output[i] = (255 - byte) as u8;
            } else if byte > 255 {
                output[i] = (byte - 255) as u8;
            } else {
                output[i] = byte as u8;
            }
        }

        return String::from_str(std::str::from_utf8(&output.to_vec()).unwrap()).unwrap();
    }
}

struct GeneticAnalyzer {}
impl Analyzer<InputType, OutputType, FeatureFlags> for GeneticAnalyzer {
    fn evaluate(&self, output: &OutputType, _params: &TestParameters<FeatureFlags>) -> f32 {
        let mut score = 0.0;
        let template = b"Hello, world!";
        let output_bytes = output.as_bytes();

        for i in 0..13 {
            if template[i] == output_bytes[i] {
                score += 1.0 / 13.0;
            }
        }

        return score;
    }
}

fn main() {
    let test_data: [Unit; 13] = [0; 13];
    let parameters: TestParameters<FeatureFlags> = TestParameters {
        generations: 1000,
        population: 5000,
        elitism_factor: 0.05,
        crossover_factor: 0.25,
        mutation_factor: 0.025,
        tournament_size: 10,
        feature_flag: Vec::new(),
    };

    let algo = GeneticAlgorithm {};
    let analyzer = GeneticAnalyzer {};

    let result = run_algorithm(
        &parameters,
        &test_data,
        &algo,
        &analyzer,
        Some(after_generation),
    );

    on_complete(
        result.score,
        &result.node.unwrap().solution,
        &result.output.unwrap(),
    );
}

fn on_complete(_score: f32, _solution: &Solution, output: &OutputType) {
    let output_value = &output;
    println!("winning output = {output_value}");
}

fn after_generation(_score: f32, _solution: &Solution, output: &OutputType) -> bool {
    let output_value = output;
    println!("{output_value}");

    if output_value.eq("Hello, world!") {
        return true;
    } else {
        return false;
    }
}
