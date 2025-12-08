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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    group: usize,
}

#[derive(Debug)]
struct Dist {
    p1: Point,
    p2: Point,
    dist: f64,
}
fn dist(p: &Point, q: &Point) -> f64 {
    let x = (p.x - q.x).pow(2);
    let y = (p.y - q.y).pow(2);
    let z = (p.z - q.z).pow(2);
    let dist = ((x + y + z) as f64).sqrt();
    return dist;
}

fn find(mut x: usize, link: &[usize]) -> usize {
    while x != link[x] {
        x = link[x]
    }
    return x;
}
fn same(a: usize, b: usize, link: &[usize]) -> bool {
    find(a, link) == find(b, link)
}

fn unite(mut a: usize, mut b: usize, link: &mut [usize], size: &mut [usize]) {
    if same(a, b, link){
        return;
    }
    a = find(a, link);
    b = find(b, link);
    if size[a] < size[b] {
        size[b] += size[a];
        link[a] = b;
    }
    else{
        size[a] += size[b];
        link[b] = a;
    }
}


pub fn p1(input: &str) -> i32 {
    let points = input
        .lines().enumerate()
        .map(|(i, l)| {
            let mut splits = l.split(",");
            Point {
                x: splits.next().unwrap().parse().unwrap(),
                y: splits.next().unwrap().parse().unwrap(),
                z: splits.next().unwrap().parse().unwrap(),
                group: i,
            }
        })
        .collect::<Vec<Point>>();

    let mut dists: Vec<Dist> = Vec::new();
    for p in points.iter() {
        for q in points.iter() {
            if p == q {
                continue;
            }

            dists.push(Dist {
                p1: *p,
                p2: *q,
                dist: dist(p, q),
            });
        }
    }
    dists.sort_by(|d1, d2| d1.dist.partial_cmp(&d2.dist).unwrap());
    dists = dists
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i.is_multiple_of(2))
        .map(|(_, v)| v)
        .collect();


    let mut dists_iter = dists.iter();

    // 1000 in assignment but 10 in test
    let mut range_end = 1000;
    if cfg!(test) {
        range_end = 10;
    }

    let n = points.len();
    let mut link = Vec::new();
    let mut size = vec![1; n];
    for i in 0..n {
        link.push(i);
    }
    for _ in 0..range_end {
        let current = dists_iter.next().unwrap();
        unite(current.p1.group, current.p2.group, &mut link, &mut size);
    }
    
    
    size = size.into_iter().enumerate().filter(|(i, _)| find(*i, &link) == *i).map(|(_, c)| c).collect::<Vec<usize>>();
    size.sort();
    size = size.into_iter().rev().collect();
    return size.iter().take(3).product::<usize>() as i32;
}

pub fn p2(input: &str) -> i64 {
    let points = input
        .lines().enumerate()
        .map(|(i, l)| {
            let mut splits = l.split(",");
            Point {
                x: splits.next().unwrap().parse().unwrap(),
                y: splits.next().unwrap().parse().unwrap(),
                z: splits.next().unwrap().parse().unwrap(),
                group: i,
            }
        })
        .collect::<Vec<Point>>();

    let mut dists: Vec<Dist> = Vec::new();
    for p in points.iter() {
        for q in points.iter() {
            if p == q {
                continue;
            }

            dists.push(Dist {
                p1: *p,
                p2: *q,
                dist: dist(p, q),
            });
        }
    }
    dists.sort_by(|d1, d2| d1.dist.partial_cmp(&d2.dist).unwrap());
    dists = dists
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i.is_multiple_of(2))
        .map(|(_, v)| v)
        .collect();


    let mut dists_iter = dists.iter();

    let n = points.len();
    let mut link = Vec::new();
    let mut size = vec![1; n];
    for i in 0..n {
        link.push(i);
    }
    let mut current = &dists[0];
    while size[find(0, &link)] < n{
        current = dists_iter.next().unwrap();
        unite(current.p1.group, current.p2.group, &mut link, &mut size);
    }

    return current.p1.x * current.p2.x;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 40;
        let example = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 25272;
        let example = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
        assert_eq!(p2(example), expected);
    }
}
