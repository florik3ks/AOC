use itertools::Itertools;
use std::cmp;
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
    input
        .lines()
        .filter_map(|l| {
            l.split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
        }) // parse lines to number tuples
        .tuple_combinations() // iterate over combinations
        .map(|(p, q)| ((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1)) // calculate distances
        .sorted()
        .next_back()
        .unwrap()
}

pub fn p2(input: &str) -> i64 {

    return 0;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 50;
        let example = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let example = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(p2(example), expected);
    }
}
