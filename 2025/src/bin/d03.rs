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

fn get_highest_index(bank: &str) -> (usize, u64){
    let mut highest_index: usize = 0;
    let mut highest_num: u64 = 0;
    let mut chars = bank.chars();
    let mut i: usize = 0;
    while let Some(c) = chars.next()  {
        let num: u64 = c.to_digit(10).unwrap().into();
        if num > highest_num{
            highest_index = i;
            highest_num = num;
        }
        i += 1;
    }
    return (highest_index, highest_num);
}

pub fn p1(input: &str) -> u64 {
    let mut joltage = 0;
    for bank in input.lines() {
        let (first_index, first_num) = get_highest_index(bank.split_at(bank.len() - 1).0);
        let (_, second_num) = get_highest_index(bank.split_at(first_index + 1).1);
        let num = (first_num * 10) + second_num;
        println!("{} {}", bank, num);
        joltage += num;
    }

    return joltage.try_into().unwrap();
}

pub fn p2(input: &str) -> u64 {
    let mut joltage = 0;
    for bank in input.lines() {
        let mut num: u64 = 0;

        let mut index = 0;
        for digit in (1..=12).rev() {
            let (i, n) = get_highest_index(&bank[index..(bank.len() - digit + 1)]);
            index += i + 1;
            num += 10u64.pow((digit - 1).try_into().unwrap()) * n;
        }
        
        println!("{} {}", bank, num);
        joltage += num;
    }

    return joltage.try_into().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 357;
        let example = r"987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 3121910778619;
        let example = r"987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(p2(example), expected);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(p2(r"987654321111111"), 987654321111);
    }
        #[test]
    fn test_part2_2() {
        assert_eq!(p2(r"811111111111119"), 811111111119);
    }
        #[test]
    fn test_part2_3() {
        assert_eq!(p2(r"234234234234278"), 434234234278);
    }
        #[test]
    fn test_part2_4() {
        assert_eq!(p2(r"818181911112111"), 888911112111);
    }



}
