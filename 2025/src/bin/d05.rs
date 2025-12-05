use std::collections::HashSet;
use std::ops::RangeInclusive;
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

fn id_in_ranges(id: i64, ranges: std::slice::Iter<'_, RangeInclusive<i64>>) -> bool {
    for range in ranges {
        if range.contains(&id) {
            // println!("{} {}-{}", id, range.start(), range.end());
            return true;
        }
    }
    return false;
}

pub fn p1(input: &str) -> i64 {
    let (range_str, ids) = input.split_once("\n\n").unwrap();

    let ranges = range_str
        .lines()
        .map(|l| {
            let mut split = l.split("-");
            RangeInclusive::new(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<RangeInclusive<i64>>>();
    
    let mut count = 0;
    for id in ids.lines().map(|id| id.parse::<i64>().unwrap()) {
        if id_in_ranges(id, ranges.iter()){
            count += 1;
        }
    }
    return count;
}

pub fn p2(input: &str) -> i64 {
    let (range_str, ids) = input.split_once("\n\n").unwrap();

    let ranges = range_str
        .lines()
        .map(|l| {
            let mut split = l.split("-");
            RangeInclusive::new(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<RangeInclusive<i64>>>();

    let mut numbers = HashSet::<i64>::new();
    

    /*
    yeah thats not the way
    
    memory allocation of 36413758519104 bytes failed
    Aborted                    (core dumped) 
    
     */

    // for range in ranges.iter(){
    //     println!("mrow");
    //     numbers.extend::<Vec<i64>>(range.clone().collect());
    // }
    // return numbers.len() as i64;
        

    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 3;
        let example = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 14;
        let example = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        assert_eq!(p2(example), expected);
    }
}
