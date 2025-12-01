use aoc2025::lines;

fn main() {

    let mut dial_number = 50;
    let mut num_zeros = 0;

    lines().map(|l| parse_line(l)).for_each(|num| {
        let n = num.signum();
        for _ in 0..num.abs() {
            dial_number += n;
            if dial_number.rem_euclid(100) == 0 {
                num_zeros += 1;
            }
        }

    });

    println!("{}", num_zeros);
}

fn parse_line(line: &str) -> i32 {
    let mut chars = line.chars();
    match chars.next() {
        Some('R') => chars.as_str().parse::<i32>().unwrap(),
        Some('L') => -chars.as_str().parse::<i32>().unwrap(),
        _ => 0
    }
}