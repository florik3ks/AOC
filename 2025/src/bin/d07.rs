use std::collections::{HashMap, HashSet};
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
    let mut lines = input.lines();

    let mut tachyons: HashSet<usize> = HashSet::new();
    // assume starting location is always in the first row
    tachyons.insert(lines.next().unwrap().find('S').unwrap());

    let mut count = 0;
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && tachyons.remove(&i) {
                // assume [i-1, i+1] are not out of bounds
                tachyons.insert(i - 1);
                tachyons.insert(i + 1);
                count += 1;
            }
        }
    }

    return count;
}

pub fn p2(input: &str) -> usize {
    let mut lines = input.lines();

    let mut tachyons: HashMap<usize, usize> = HashMap::new();
    // assume starting location is always in the first row
    tachyons.insert(lines.next().unwrap().find('S').unwrap(), 1);

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '^'
                && let Some(t) = tachyons.remove(&i)
            {
                tachyons.insert(i + 1, t + tachyons.get(&(i + 1)).unwrap_or(&0));
                tachyons.insert(i - 1, t + tachyons.get(&(i - 1)).unwrap_or(&0));
            }
        }
    }

    return tachyons.values().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 21;
        let example = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 40;
        let example = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(p2(example), expected);
    }
}
