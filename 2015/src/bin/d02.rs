use std::time::Instant;
use std::{fs::File, io::Read};

use itertools::Itertools;

fn main() {
    // get day number from file
    let day = file!().split('/').next_back().unwrap()[1..3].to_owned();

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
    input
        .lines()
        .map(|line| {
            let (l, w, h) = line
                .split("x")
                .map(|v| v.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            2 * l * w + 2 * w * h + 2 * h * l + std::cmp::min(l * w, std::cmp::min(w * h, h * l))
        })
        .sum::<i32>()
}

pub fn p2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (l, w, h) = line
                .split("x")
                .map(|v| v.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap();
            l * w * h + 2 * std::cmp::min(l + w, std::cmp::min(w + h, h + l))
        })
        .sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(p1("2x3x4"), 58);
        assert_eq!(p1("1x1x10"), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(p2("2x3x4"), 34);
        assert_eq!(p2("1x1x10"), 14);
    }
}
