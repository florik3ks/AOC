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

pub fn p1(input: &str) -> i64 {
    let mut lines = input.lines().map(|line| line.split_ascii_whitespace());
    // take out operator list from lines
    let operators: Vec<&str> = lines.next_back().unwrap().collect();
    // convert lines to numbers
    let num_lines = lines.map(|line| line.map(|value| value.parse::<i64>().unwrap()));

    let mut column_subresults: Vec<i64> = Vec::new();
    for operator in operators.iter().copied() {
        column_subresults.push(match operator {
            "+" => 0,
            "*" => 1,
            _ => {
                panic!("??"); // should never happen
            }
        });
    }
    for row in num_lines {
        for (col, value) in row.enumerate() {
            column_subresults[col] = match operators[col] {
                "+" => column_subresults[col] + value,
                "*" => column_subresults[col] * value,
                _ => {
                    panic!("??"); // should never happen
                }
            }
        }
    }
    return column_subresults.iter().sum();
}

pub fn p2(input: &str) -> i64 {
    let mut lines = input.lines();
    // take out operator list from lines
    let operators: Vec<&str> = lines
        .next_back()
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    let char_lines: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut subresults: Vec<i64> = Vec::new();
    for operator in operators.iter().copied() {
        subresults.push(match operator {
            "+" => 0,
            "*" => 1,
            _ => {
                panic!("??"); // should never happen
            }
        });
    }

    let mut index = 0;
    // transpose input and then handle it line by line
    for i in 0..char_lines[0].len() {
        let line = char_lines
            .iter()
            .map(|row| row[i]) // turn row to column
            .collect::<String>()
            .trim()
            .parse::<i64>();

        if let Ok(num) = line {
            subresults[index] = match operators[index] {
                "+" => subresults[index] + num,
                "*" => subresults[index] * num,
                _ => {
                    panic!("??"); // should never happen
                }
            }
        } else {
            index += 1;
        }
    }

    return subresults.iter().sum();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 4277556;
        let example = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 3263827;
        let example = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(p2(example), expected);
    }
}
