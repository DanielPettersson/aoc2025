use aoc2025::lines;
use crate::Thing::{Beam, Empty, Source, Splitter};

fn main() {
    let map: Vec<Vec<Thing>> = parse();
    print_map(&map);
    let res = iterate(Iteration {
        map,
        num_splits: 0,
    });
    println!("{}", res.num_splits);
}

fn parse() -> Vec<Vec<Thing>> {
    lines()
        .map(|l| l.chars().map(Thing::from).collect())
        .collect()
}

fn iterate(input: Iteration) -> Iteration {
    let in_map = input.map;
    let height = in_map.len();
    let width = in_map[0].len();
    let mut map = in_map.clone();
    let mut num_splits = input.num_splits;

    for (y, row) in in_map.iter().enumerate() {
        for (x, thing) in row.iter().enumerate() {
            let up: Option<&Thing> = if y > 0 { Some(&in_map[y - 1][x]) } else { None };
            let down: Option<&Thing> = if y < height - 1 { Some(&in_map[y + 1][x]) } else { None };
            let left: Option<&Thing> = if x > 0 { Some(&in_map[y][x - 1]) } else { None };
            let right: Option<&Thing> = if x < width - 1 { Some(&in_map[y][x + 1]) } else { None };

            match thing {
                Source => {
                    if down.is_some_and(|d| *d == Empty) {
                        map[y + 1][x] = Beam;
                    }
                }
                Splitter => {
                    if up.is_some_and(|d| *d == Beam) {
                        let mut split = false;
                        if left.is_some_and(|d| *d == Empty) {
                            map[y][x - 1] = Beam;
                            split = true;
                        }
                        if right.is_some_and(|d| *d == Empty) {
                            map[y][x + 1] = Beam;
                            split = true;
                        }
                        if split {
                            num_splits += 1;
                        }
                    }
                }
                Beam => {
                    if down.is_some_and(|d| *d == Empty) {
                        map[y + 1][x] = Beam;
                    }
                }
                Empty => {}
            }
        }
    }

    print_map(&map);

    if in_map != map {
        iterate(Iteration {
            num_splits,
            map,
        })
    } else {
        Iteration {
            num_splits,
            map,
        }
    }
}

fn print_map(map: &[Vec<Thing>]) {
    for row in map {
        println!(
            "{}",
            row.iter()
                .map(|thing| thing.into())
                .collect::<Vec<&str>>()
                .join("")
        );
    }
    println!("***********************************");
}

struct Iteration {
    num_splits: u32,
    map: Vec<Vec<Thing>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Thing {
    Source,
    Splitter,
    Beam,
    Empty,
}

impl From<&Thing> for &str {
    fn from(value: &Thing) -> Self {
        match value {
            Source => "S",
            Splitter => "^",
            Beam => "|",
            Empty => ".",
        }
    }
}

impl From<char> for Thing {
    fn from(c: char) -> Self {
        match c {
            'S' => Source,
            '^' => Splitter,
            '|' => Beam,
            '.' => Empty,
            _ => unimplemented!("Unknown thing: {}", c),
        }
    }
}
