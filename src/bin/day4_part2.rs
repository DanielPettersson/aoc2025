use aoc2025::lines;

fn main() {
    let mut sum = 0;
    let mut num_removed;

    let mut map: Vec<Vec<bool>> = lines()
        .map(|l| l.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect();

    loop {
        (map, num_removed) = remove(&mut map);
        sum += num_removed;
        if num_removed == 0 {
            break;
        }
    }

    println!("{}", sum);
}

fn remove(map: &[Vec<bool>]) -> (Vec<Vec<bool>>, u32) {
    let mut sum = 0;
    let height = map.len();
    let width = map.iter().next().unwrap().len();
    let mut new_map = Vec::with_capacity(height * width);

    for y in 0..height {
        new_map.push(vec![false; width]);
        for x in 0..width {
            if map[y][x] && count_neighbors(&map, x, y) < 4 {
                sum += 1;
                new_map[y][x] = false;
            } else {
                new_map[y][x] = map[y][x];
            }
        }
    }
    (new_map, sum)
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
