use std::ops::{RangeInclusive};
use aoc2025::lines;

fn main() {
    let mut sum = 0;

    let (ranges, ids) = parse_input();

    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn parse_input() -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut parsing_ranges = true;
    let mut ids: Vec<u64> = Vec::new();
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    lines().for_each(|line| {
        if line.is_empty() { parsing_ranges = false; return; }
        if parsing_ranges {
            let mut split = line.split("-");
            let a = split.next().unwrap().parse::<u64>().unwrap();
            let b = split.next().unwrap().parse::<u64>().unwrap();
            ranges.push(a..=b);
        } else {
            ids.push(line.parse::<u64>().unwrap());
        }
    });
    (ranges, ids)
}
