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

fn is_valid_p1(num: i64) -> bool {
    let num_str = num.to_string();
    if num_str.len() % 2 != 0 {
        return false;
    }
    let (fst, snd) = num_str.split_at(num_str.len() / 2);
    return fst == snd;
}

pub fn p1(input: &str) -> i64 {
    let mut result = 0;
    let ranges = input.split(",");
    for range in ranges {
        let mut split = range.split("-");
        let fst_num = split.next().unwrap().parse::<i64>().unwrap();
        let snd_num = split.next().unwrap().parse::<i64>().unwrap();
        for i in fst_num..=snd_num {
            if is_valid_p1(i) {
                result += i;
            }
        }
    }

    return result;
}

fn is_valid_p2(num: i64) -> bool {
    let num_str = num.to_string();

    for i in 1..=num_str.len() / 2 {
        let (split, mut remain) = num_str.split_at(i);
        let mut valid = true;
        while valid && remain.len() >= i {
            let slice: &str;
            (slice, remain) = remain.split_at(i);
            if slice != split {
                valid = false;
            }
        }
        if valid && remain.len() == 0 {
            return true;
        }
    }
    return false;
}

pub fn p2(input: &str) -> i64 {
    let mut result = 0;
    let ranges = input.split(",");
    for range in ranges {
        let mut split = range.split("-");
        let fst_num = split.next().unwrap().parse::<i64>().unwrap();
        let snd_num = split.next().unwrap().parse::<i64>().unwrap();
        for i in fst_num..=snd_num {
            if is_valid_p2(i) {
                result += i;
            }
        }
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 1227775554;
        let example = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 4174379265;
        let example = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(p2(example), expected);
    }
}
