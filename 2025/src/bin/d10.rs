use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use std::vec;
use std::{fs::File, io::Read};

use faer::Side;
use faer::col::generic::Col;
use faer::prelude::*;
use faer::sparse::*;
use regex::Regex;

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

const INPUT_REGEX: &str = r"\[(?<lights>.+)\]\s(?<buttons>.+)\s\{(?<joltages>.+)\}"; //\s(?<buttons>((\(\d+(,\d+)*\)\s))+)\s\{(?<joltage>.+)\}";
pub fn p1(input: &str) -> i32 {
    let re = Regex::new(INPUT_REGEX).unwrap();

    let parsed: Vec<(i16, Vec<i16>)> = input
        .lines()
        .map(|l| {
            let Some(captures) = re.captures(l) else {
                panic!("input parsing failed");
            };

            let lights = &captures["lights"];
            let buttons = &captures["buttons"];
            let _ = &captures["joltages"];
            let light_len = lights.len() as u32;
            (
                lights
                    .chars()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| match c {
                        '#' => 2_i16.pow(i.try_into().unwrap()),
                        _ => 0,
                    })
                    .sum(),
                buttons
                    .split(" ")
                    .map(|b| {
                        let mut chars = b.chars();
                        // drop the ()
                        chars.next();
                        chars.next_back();
                        chars
                            .map(|c| match c {
                                ',' => 0,
                                _ => 2_i16.pow(light_len - 1 - c.to_digit(10).unwrap()),
                            })
                            .sum()
                    })
                    .collect(),
            )
        })
        .collect();

    let mut result = 0;
    for (lights, buttons) in parsed {
        result += find_combination(lights, buttons);
    }

    return result;
}

fn find_combination(lights: i16, buttons: Vec<i16>) -> i32 {
    let mut queue: VecDeque<(i32, i16)> = VecDeque::new();
    let mut computed: HashSet<i16> = HashSet::new();
    computed.insert(0);
    queue.push_back((0, 0));
    while !queue.is_empty() {
        let (count, current) = queue.pop_front().unwrap();
        for b in buttons.iter() {
            let value = current ^ b;
            if !computed.contains(&value) {
                computed.insert(value);
                queue.push_back((count + 1, value));
                if value == lights {
                    return count + 1;
                }
            }
        }
    }
    panic!("mrow");
}

pub fn p2(input: &str) -> i32 {
    let re = Regex::new(INPUT_REGEX).unwrap();

    let parsed: Vec<(Vec<Vec<usize>>, Vec<f64>)> = input
        .lines()
        .map(|l| {
            let Some(captures) = re.captures(l) else {
                panic!("input parsing failed");
            };

            let _ = &captures["lights"];
            let buttons = &captures["buttons"];
            let joltages = &captures["joltages"];
            (
                buttons
                    .split(" ")
                    .map(|b| {
                        let mut chars = b.chars();
                        // drop the ()
                        chars.next();
                        chars.next_back();
                        chars
                            .filter_map(|c| match c {
                                ',' => None,
                                _ => Some(c.to_digit(10).unwrap() as usize),
                            })
                            .collect()
                    })
                    .collect(),
                joltages
                    .split(",")
                    .map(|v| v.trim().parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let mut result = 0;
    for (buttons, j) in parsed {
        println!("{} {}", j.len(), buttons.len());
        let mut a: mat::generic::Mat<mat::Own<f64>> = Mat::zeros(j.len(), buttons.len());

        // for each row of the matrix
        for (b, button) in buttons.iter().enumerate() {
            // println!("{:?}\t{b}", button);
            for row in 0..j.len() {
                for (i, num) in button.iter().enumerate() {
                    if *num == row {
                        // println!("button {b} in row {row} in col {i}");
                        *a.get_mut(row, b) = 1_f64;
                    }
                }
            }
        }
        // (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        // A(3) B(1,3) C(2) D(2,3) E(0,2) F(0,1) {3,5,4,7}
        // 3 = E + F        [0,0,0,0,1,1]x = [3]
        // 5 = B + F        [0,1,0,0,0,1]x = [5]
        // 4 = C + D + E    [0,0,1,1,1,0]x = [4]
        // 7 = A + B + D    [1,1,0,1,0,0]x = [7]

        // A=1 B=3 C=0 D=3 E=1 F=2
        // 3 = 1 + 2
        // 5 = 3 + 2
        // 4 = 0 + 3 + 1
        // 7 = 1 + 3 +3

        let b= Col::from_iter(j.clone().into_iter());
        println!("{a:?}");
        println!("{b:?}");

        // A^T * Ax = A^T * b
        // minimizes x
        let ata = a.transpose() * &a;
        let atb = a.transpose() * b;
        

        let lr = ata.full_piv_lu();
        let x = lr.solve(&atb);
        println!("{x:?}");
    }

    return 0;
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 7;
        let example = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 33;
        let example = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(p2(example), expected);
    }
}
