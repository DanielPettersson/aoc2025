use aoc2025::lines;

fn main() {
    let mut sum = 0;

    let map: Vec<Vec<bool>> = lines()
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect();

    let height = map.len();
    let width = map.iter().next().unwrap().len();

    for x in 0..width {
        for y in 0..height {
            if map[y][x] && count_neighbors(&map, x, y) < 4 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}

fn count_neighbors(map: &[Vec<bool>], x: usize, y: usize) -> usize {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;
            if xx < 0 || xx >= map[0].len() as i32 || yy < 0 || yy >= map.len() as i32 {
                continue;
            }
            if map[yy as usize][xx as usize] {
                count += 1;
            }
        }
    }
    count
}
