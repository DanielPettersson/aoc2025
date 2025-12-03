use aoc2025::lines;

fn main() {

    let sum: u64 = lines().map(|line| {
        let l1 = &line[..line.len() - 1];
        let (x, c1) = l1.chars().rev().enumerate().max_by_key(|x| x.1).unwrap();
        let l2 = &line[line.len() - (x + 1)..];
        let c2 = l2.chars().max().unwrap();
        format!("{}{}", c1, c2).parse::<u64>().unwrap()
    }).sum();

    println!("{}", sum);
}
