use crate::Operation::{Add, Multiply};
use aoc2025::lines;

fn main() {
    let math_problems = parse();
    let sum: u64 = math_problems.iter().map(|p| p.calculate()).sum();
    println!("{}", sum);
}

fn parse() -> Vec<MathProblem> {
    let mut math_problems: Vec<MathProblem> = lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(MathProblem::from)
        .collect();

    lines().for_each(|l| {
        l.split_whitespace().enumerate().for_each(|(i, num_str)| {
            if let Ok(num) = num_str.parse::<u64>() {
                math_problems[i].numbers.push(num);
            }
        })
    });

    math_problems
}

struct MathProblem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl MathProblem {
    fn new(operation: Operation) -> Self {
        Self {
            numbers: Vec::new(),
            operation,
        }
    }

    fn calculate(&self) -> u64 {
        self.numbers
            .iter()
            .fold(self.operation.initial_fold_value(), |acc, num| {
                self.operation.apply(acc, *num)
            })
    }
}

impl From<&str> for MathProblem {
    fn from(value: &str) -> Self {
        match value {
            "*" => MathProblem::new(Multiply),
            "+" => MathProblem::new(Add),
            _ => unimplemented!("unknown operation"),
        }
    }
}

enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn initial_fold_value(&self) -> u64 {
        match self {
            Add => 0,
            Multiply => 1,
        }
    }
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Add => a + b,
            Multiply => a * b,
        }
    }
}
