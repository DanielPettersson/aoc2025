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
                .map(|x| if is_repeated(x) { x } else { 0 })
                .sum::<u64>()
        })
        .sum();

    println!("{}", sum);
}

fn is_repeated(x: u64) -> bool {
    let num_str = x.to_string();
    let half_len = num_str.len() / 2;
    for i in 1..=half_len {
        let subs = num_str
            .as_bytes()
            .chunks(i)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        if is_all_same(&subs) {
            return true;
        }
    }
    false
}

fn is_all_same(parts: &[&str]) -> bool {
    if parts.is_empty() {
        return false;
    }

    let first = parts[0];
    parts.iter().all(|&p| p == first)
}
