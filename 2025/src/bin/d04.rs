use std::time::Instant;
use std::{fs::File, io::Read};

fn main() {
    // get day number from file
    let day = file!().split('/').last().unwrap()[1..3].to_owned();

    // read input
    let path = format!("input/input{day}.txt");
    println!("{}", path);
    let mut file = File::open(path).expect("File not found");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("cannot read file");

    // measure time needed to run both parts
    let mut now: Instant;
    let mut elapsed: std::time::Duration;

    now = Instant::now();
    let p1result = p1(&input);
    elapsed = now.elapsed();

    println!("Part 1 solution in: {:.2?}", elapsed);
    println!("{}", p1result);

    now = Instant::now();
    let p2result = p2(&input);
    elapsed = now.elapsed();

    println!("Part 2 solution in: {:.2?}", elapsed);
    println!("{}", p2result);
}

pub fn p1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|s| s.trim().chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }

            let mut rolls = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x.checked_add_signed(dx);
                    let ny = y.checked_add_signed(dy);

                    if nx.is_none() || ny.is_none() {
                        continue;
                    }

                    if let Some(row) = grid.get(ny.unwrap())
                        && let Some(c) = row.get(nx.unwrap())
                        && *c == '@'
                    {
                        rolls += 1;
                    }
                }
            }
            if rolls < 4 {
                count += 1;
            }
        }
    }
    return count;
}

fn remove_rolls_of_paper(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut changes = 0;
    let mut grid_copy = grid.clone();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }

            let mut rolls = 0;
            for dx in -1_isize..=1 {
                for dy in -1_isize..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x.checked_add_signed(dx);
                    let ny = y.checked_add_signed(dy);

                    if nx.is_none() || ny.is_none() {
                        continue;
                    }

                    if let Some(row) = grid.get(ny.unwrap())
                        && let Some(c) = row.get(nx.unwrap())
                        && *c == '@'
                    {
                        rolls += 1;
                    }
                }
            }
            if rolls < 4 {
                grid_copy[y][x] = '.';
                changes += 1;
            }
        }
    }

    *grid = grid_copy;
    return changes;
}

pub fn p2(input: &str) -> i32 {
    let mut grid = input
        .lines()
        .map(|s| s.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut count = 0;

    let mut changes = true;
    while changes {
        changes = false;
        let removed = remove_rolls_of_paper(&mut grid);
        count += removed;
        if removed > 0 {
            changes = true;
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 13;
        let example = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 43;
        let example = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
        assert_eq!(p2(example), expected);
    }
}
