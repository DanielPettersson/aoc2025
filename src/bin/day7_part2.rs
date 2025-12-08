use crate::Thing::{Empty, Source, Splitter};
use aoc2025::lines;

fn main() {
    let map: Vec<Vec<Thing>> = parse();
    let height = map.len();
    let width = map[0].len();
    let pos = find_source(&map);

    let mut timelines = vec![vec![0u64; width]; height];
    timelines[pos.1][pos.0] = 1;

    for (y, line) in map.iter().enumerate() {
        if y == height - 1 {
            continue;
        }
        for (x, thing) in line.iter().enumerate() {
            let curr = timelines[y][x];
            if thing == &Splitter {
                timelines[y + 1][x - 1] += curr;
                timelines[y + 1][x + 1] += curr;
            } else {
                timelines[y + 1][x] += curr;
            }
        }
    }
    let sum = timelines.last().unwrap().iter().sum::<u64>();
    println!("{}", sum);
}

fn parse() -> Vec<Vec<Thing>> {
    lines()
        .map(|l| l.chars().map(Thing::from).collect())
        .collect()
}

fn find_source(map: &[Vec<Thing>]) -> (usize, usize) {
    let mut pos: (usize, usize) = (0, 0);
    for (y, line) in map.iter().enumerate() {
        for (x, thing) in line.iter().enumerate() {
            if thing == &Source {
                pos = (x, y);
            }
        }
    }
    pos
}

#[derive(Eq, PartialEq)]
enum Thing {
    Source,
    Splitter,
    Empty,
}

impl From<char> for Thing {
    fn from(c: char) -> Self {
        match c {
            'S' => Source,
            '^' => Splitter,
            '.' => Empty,
            _ => unimplemented!("Unknown thing: {}", c),
        }
    }
}
