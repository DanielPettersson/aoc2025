use aoc2025::lines;

fn main() {
    let sum: u64 = lines()
        .next()
        .unwrap()
        .split(',')
        .map(|range| {
            let mut split = range.split('-');
            let r1 = split.next().unwrap().parse::<u64>().unwrap();
            let r2 = split.next().unwrap().parse::<u64>().unwrap();
            (r1..=r2)
                .map(|x| {
                    let num_str = x.to_string();
                    let l_half = num_str.len() / 2;
                    let x1 = &num_str[..l_half];
                    let x2 = &num_str[l_half..];
                    if x1 == x2 { x } else { 0 }
                }).sum::<u64>()
        })
        .sum();

    println!("{}", sum);
}
