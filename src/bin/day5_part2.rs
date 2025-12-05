use aoc2025::lines;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    let ranges = merge_overlapping_ranges(parse_input());
    let sum: u64 = ranges.iter().map(|r| r.end() - r.start() + 1).sum();
    println!("{}", sum);
}

fn merge_overlapping_ranges(ranges: HashSet<RangeInclusive<u64>>) -> HashSet<RangeInclusive<u64>> {
    let mut ret: HashSet<RangeInclusive<u64>> = HashSet::new();
    for (i, range1) in ranges.iter().enumerate() {
        let mut r = range1.clone();
        for range2 in ranges.iter().skip(i + 1) {
            if is_overlapping(&r, &range2) {
                r = RangeInclusive::new(
                    *min(r.start(), range2.start()),
                    *max(r.end(), range2.end()),
                );
            }
        }
        ret.insert(r);
    }
    if is_overlapping_ranges(&ret) {
        return merge_overlapping_ranges(ret);
    }
    ret
}

fn is_overlapping(range1: &RangeInclusive<u64>, range2: &RangeInclusive<u64>) -> bool {
    range1 != range2 && range1.start() <= range2.end() && range2.start() <= range1.end()
}

fn is_overlapping_ranges(ranges: &HashSet<RangeInclusive<u64>>) -> bool {
    for range1 in ranges {
        for range2 in ranges {
            if is_overlapping(range1, range2) {
                return true;
            }
        }
    }
    false
}

fn parse_input() -> HashSet<RangeInclusive<u64>> {
    let mut parsing_ranges = true;
    let mut ranges: HashSet<RangeInclusive<u64>> = HashSet::new();
    lines().for_each(|line| {
        if line.is_empty() {
            parsing_ranges = false;
            return;
        }
        if parsing_ranges {
            let mut split = line.split("-");
            let a = split.next().unwrap().parse::<u64>().unwrap();
            let b = split.next().unwrap().parse::<u64>().unwrap();
            ranges.insert(a..=b);
        }
    });
    ranges
}
