use crate::Operation::{Add, Multiply};
use aoc2025::lines;

fn main() {
    let math_problems = parse();
    let sum: u64 = math_problems.iter().map(|p| p.calculate()).sum();
    println!("{}", sum);
}

fn parse() -> Vec<MathProblem> {
    let cols = lines().next().unwrap().chars().count();
    let mut ret: Vec<MathProblem> = Vec::with_capacity(cols);

    let mut m = MathProblem::new();
    for i in 0..cols {
        let mut num_str = String::new();
        for l in lines() {
            let c = l.chars().nth(i).unwrap();
            if let Ok(_) =  c.to_string().parse::<u8>() {
                num_str.push(c);
            }
            if c == '*' {
                m.set_operation(Multiply);
            }
            if c == '+' {
               m.set_operation(Add);
            }
        }
        if num_str.is_empty() {
            ret.push(m.clone());
            m = MathProblem::new();
        } else {
            m.add_number(num_str.parse().unwrap());
        }
    }
    ret.push(m);

    ret
}

#[derive(Clone)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: Operation,
}

impl MathProblem {
    fn new() -> Self {
        Self {
            numbers: Vec::new(),
            operation: Add,
        }
    }

    fn set_operation(&mut self, operation: Operation) {
        self.operation = operation;
    }

    fn add_number(&mut self, number: u64) {
        self.numbers.push(number);
    }

    fn calculate(&self) -> u64 {
        self.numbers
            .iter()
            .fold(self.operation.initial_fold_value(), |acc, num| {
                self.operation.apply(acc, *num)
            })
    }
}

#[derive(Clone)]
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
