use aoc2025::lines;

fn main() {

    let sum: u64 = lines().map(|line| {
        let mut cs = "".to_string();
        let mut x = 0;
        for i in 0..12 {
            let end = line.len() - (12 - i) + 1;
            let l = &line[x..end];
            let (xx, c) = l.chars().rev().enumerate().max_by_key(|x| x.1).unwrap();
            x += l.len() - xx;
            cs.push_str(&c.to_string());
        }
        cs.parse::<u64>().unwrap()
    }).sum();

    println!("{}", sum);
}
