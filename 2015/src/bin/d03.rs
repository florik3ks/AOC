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

pub fn p1(input: &str) -> usize {
    input
        .chars()
        .scan((0, 0), |state, x| {
            match x {
                '^' => state.1 += 1,
                'v' => state.1 -= 1,
                '>' => state.0 += 1,
                '<' => state.0 -= 1,
                _ => (),
            };
            Some(*state)
        })
        .chain(std::iter::once((0, 0)))
        .unique()
        .count()
}

pub fn p2(input: &str) -> usize {
    let (santa, robo): (Vec<char>, Vec<char>) = input.chars().tuples().unzip();
    std::iter::once((0, 0))
        .chain(santa.into_iter().scan((0, 0), |state, x| {
            match x {
                '^' => state.1 += 1,
                'v' => state.1 -= 1,
                '>' => state.0 += 1,
                '<' => state.0 -= 1,
                _ => (),
            };
            Some(*state)
        }))
        .chain(robo.into_iter().scan((0, 0), |state, x| {
            match x {
                '^' => state.1 += 1,
                'v' => state.1 -= 1,
                '>' => state.0 += 1,
                '<' => state.0 -= 1,
                _ => (),
            };
            Some(*state)
        }))
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(p1("^v^v^v^v^v"), 2);
        assert_eq!(p1(">"), 2);
        assert_eq!(p1("^>v<"), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(p2("^v"), 3);
        assert_eq!(p2("^>v<"), 3);
        assert_eq!(p2("^v^v^v^v^v"), 11);
    }
}
