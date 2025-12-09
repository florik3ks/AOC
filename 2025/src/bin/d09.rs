use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::time::Instant;
use std::{fs::File, io::Read};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

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

fn visualize(ordered_corners: &[(i64, i64)], (min_x, min_y, max_x, max_y): (i64, i64, i64, i64)) -> Result<(), std::io::Error> {
    let mut meow = ordered_corners.iter();
    let mut data = Data::new().move_to(*meow.next().unwrap());
    for meowmeow in meow{
        data = data.line_to(*meowmeow);
    }
    data = data.close();

    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "pink")
        .set("stroke-width", 100)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (min_x - 1, min_y - 1, max_x + 1, max_y + 1))
        .add(path);

    svg::save("meow.svg", &document)
}

pub fn p2(input: &str) -> i64 {
    let corners: Vec<(i64, i64)> = input
        .lines()
        .filter_map(|l| {
            l.split(",")
                .map(|v| v.parse::<i64>().unwrap())
                .collect_tuple::<(i64, i64)>()
        })
        .collect();

    let min_x = corners.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = corners.iter().max_by_key(|(x, _)| x).unwrap().0;

    let min_y = corners.iter().min_by_key(|(_, y)| y).unwrap().1;
    let max_y = corners.iter().max_by_key(|(_, y)| y).unwrap().1;

    let mut edges: HashSet<(i64, i64)> = HashSet::new();
    for (&(x1, y1), &(x2, y2)) in corners.iter().circular_tuple_windows() {
        if x1 != x2 {
            for x in min(x1, x2)..max(x1, x2) {
                edges.insert((x, y1));
            }
        }
        if y1 != y2 {
            for y in min(y1, y2)..max(y1, y2) {
                edges.insert((x1, y));
            }
        }
    }
    println!("{} {} {} {}", min_x, min_y, max_x, max_y);
    println!("{}", (max_x - min_x) * (max_y - min_y));
    
    visualize(&corners, (min_x, min_y, max_x, max_y)).unwrap();
    
    let corner_hash: HashSet<(i64, i64)> = corners.into_iter().collect();

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
        let expected = 24;
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
