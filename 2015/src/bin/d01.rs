use std::time::Instant;
use std::{fs::File, io::Read};

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
    input.chars().filter(|c| *c == '(').count() as i32
        - input.chars().filter(|c| *c == ')').count() as i32
}

pub fn p2(input: &str) -> i32 {
    input
        .chars()
        .scan(0, |state, x| {
            *state += if x == '(' { 1 } else { -1 };
            if *state == -1 {
                return None;
            }
            Some(*state)
        })
        .count() as i32
        + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(p1("(())"), 0);
        assert_eq!(p1("()()"), 0);
        assert_eq!(p1("(()(()("), 3);
        assert_eq!(p1(")())())"), -3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(p2(")"), 1);
        assert_eq!(p2("()())"), 5);
    }
}
