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

#[derive(Clone)]
struct StringSolution {
    shifts: InputType,
}

#[derive(Clone)]
struct AlgorithmOutput {
    content: OutputType,
}

struct StringAlgorithm {}

impl Algorithm<InputType, OutputType, StringSolution> for StringAlgorithm {
    fn allocate_node(&self, params: &TestParameters) -> Node<StringSolution> {
        let mut solution = StringSolution { shifts: [0; 13] };

        return Node {
            id: 0,
            score: f32::MIN,
            solution: solution,
        };
    }

    fn combine_node(
        &self,
        left: Node<StringSolution>,
        right: Node<StringSolution>,
        params: &TestParameters,
    ) -> Node<StringSolution> {
        let mut rng = rand::thread_rng();
        let mut next_solution: InputType = [0; 13];

        for i in 0..13 {
            if rng.gen_bool(1.0 / 2.0) {
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
            solution: StringSolution {
                shifts: next_solution,
            },
        };
    }

    fn output(
        &self,
        node: &mut Node<StringSolution>,
        input: &InputType,
        params: &TestParameters,
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

struct StringAnalyzer {}
impl Analyzer<InputType, OutputType> for StringAnalyzer {
    fn evaluate(&self, input: &InputType, output: OutputType, params: &TestParameters) -> f32 {
        let mut score = 0.0;
        let template = b"Hello, world!";
        let output_bytes = output.as_bytes();

        for i in 0..13 {
            if template[i] == output_bytes[i] {
                score += 1.0 / 12.0;
            }
        }

        return score;
    }
}

fn main() {
    let test_data: [Unit; 13] = [0; 13];
    let parameters: TestParameters = TestParameters {
        generations: 1000,
        population: 50000,
        elitism_factor: 0.3,
        crossover_factor: 0.25,
        mutation_factor: 0.15,
        tournament_size: 6,
        feature_flage: Vec::new(),
    };

    let algo = StringAlgorithm {};
    let analyzer = StringAnalyzer {};

    run_algorithm(
        &parameters,
        test_data,
        algo,
        analyzer,
        Some(after_generation),
    );
}

fn after_generation(output: OutputType) -> bool {
    println!("{output}");

    if output.eq("Hello, world!") {
        return true;
    } else {
        return false;
    }
}
