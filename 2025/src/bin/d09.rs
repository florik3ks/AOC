use std::cmp;
use std::{fs::File, io::Read};
use std::time::Instant;


fn main(){
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


pub fn p1(input: &str) -> i64{
    let points: Vec<_> = input.lines().map(|l| {
        let mut a = l.split(",").map(|v| v.parse::<i64>().unwrap());
        (a.next().unwrap(), a.next().unwrap())
    }
    ).collect();

    let mut dists: Vec<(i64, (i64, i64), (i64, i64))> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        for q in points.split_at(i + 1).1.iter() {
            let x = cmp::max(p.0, q.0) - cmp::min(p.0, q.0);
            let y = cmp::max(p.1, q.1) - cmp::min(p.1, q.1);
            let dist: i64 = (x + 1) * (y + 1);
            dists.push((dist, *p, *q));
        }
    }
    dists.sort_by_key(|v| v.0);
    return dists.last().unwrap().0;
}

pub fn p2(input: &str) -> i64{


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
        let expected = 50;
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
