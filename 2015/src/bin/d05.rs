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
    input.lines().filter(|line| {
        line.chars().tuple_windows().any(|(t1, t2)| t1 == t2)
        && !line.chars().tuple_windows().any(|(t1, t2)| ["ab", "cd", "pq", "xy"].contains(&format!("{t1}{t2}").as_str()))
        && line.chars().filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x)).count() >= 3
    }).count()
}

pub fn p2(input: &str) -> usize {
    input.lines().filter(|line| {
        line.chars().tuple_windows().any(|(t1, _, t3)| t1 == t3)
        && line.chars().tuple_windows::<(_, _)>().enumerate().tuple_combinations().any(|((i1, t1), (i2, t2))|{
            i2 - i1 != 1 && t1 == t2
        })
    }).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(p1("ugknbfddgicrmopn"), 1);
        assert_eq!(p1("aaa"), 1);
        assert_eq!(p1("jchzalrnumimnmhp"), 0);
        assert_eq!(p1("haegwjzuvuyypxyu"), 0);
        assert_eq!(p1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(p2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(p2("xxyxx"), 1);
        assert_eq!(p2("uurcxstgmygtbstg"), 0);
        assert_eq!(p2("ieodomkazucvgmuy"), 0);
    }
}
