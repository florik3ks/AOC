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
    let mut score: i32 = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        let direction: char = line.chars().nth(0).unwrap();
        let num: i32 = line.split_at(1).1.parse::<i32>().unwrap();

        match direction {
            'L' => {
                dial -= num;
            }
            'R' => {
                dial += num;
            }
            _ => {}
        }

        // dial is negative between loops but idk
        // rust modulo is weird but it still works?
        dial %= 100;

        if dial == 0 {
            score += 1;
        }
    }
    return score;
}

pub fn p2(input: &str) -> i32 {
    let mut score: i32 = 0;
    let mut dial: i32 = 50;
    for line in input.lines() {
        let direction: char = line.chars().nth(0).unwrap();
        let num: i32 = line.split_at(1).1.parse::<i32>().unwrap();
        match direction {
            'L' => {
                // i'm sorry to whoever reads this
                for _ in 0..num {
                    dial -= 1;
                    dial %= 100;
                    if dial == 0 {
                        score += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..num {
                    dial += 1;
                    dial %= 100;
                    if dial == 0 {
                        score += 1;
                    }
                }
            }
            _ => {}
        }
    }
    return score;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 3;
        let example = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 6;
        let example = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(p2(example), expected);
    }
}
