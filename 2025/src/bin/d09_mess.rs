use itertools::{Itertools, all};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::Rem;
use std::thread::sleep;
use std::time::{Duration, Instant};
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

fn rectangle_valid(
    edges: &HashSet<(i64, i64)>,
    corners: &HashSet<(i64, i64)>,
    (x1, y1): (i64, i64),
    (x2, y2): (i64, i64),
    minimum_x_border: i64,
    maximum_x_border: i64,
    minimum_y_border: i64,
    maximum_y_border: i64,
    ordered_corners: &[(i64, i64)],
    (min_x1, min_y1, max_x1, max_y1): (i64, i64, i64, i64),
) -> bool {
    let min_x = min(x1, x2);
    let max_x = max(x1, x2);
    let min_y = min(y1, y2);
    let max_y = max(y1, y2);

    let mut e1 = Vec::new();
    let mut e2 = Vec::new();
    let mut e3 = Vec::new();
    let mut e4 = Vec::new();
    for x in min_x..=max_x {
        e1.push((x, min_y));
        e2.push((x, max_y));
    }
    for y in min_y..=max_y {
        e3.push((min_x, y));
        e4.push((max_x, y));
    }

    // visualize_with_points(
    //     "aa/e1.svg",
    //     ordered_corners,
    //     (min_x1, min_y1, max_x1, max_y1),
    //     vec![e1.clone()],
    //     "purple",
    // );
    // visualize_with_points(
    //     "aa/e2.svg",
    //     ordered_corners,
    //     (min_x1, min_y1, max_x1, max_y1),
    //     vec![e2.clone()],
    //     "purple",
    // );
    // visualize_with_points(
    //     "aa/e3.svg",
    //     ordered_corners,
    //     (min_x1, min_y1, max_x1, max_y1),
    //     vec![e3.clone()],
    //     "purple",
    // );
    // visualize_with_points(
    //     "aa/e4.svg",
    //     ordered_corners,
    //     (min_x1, min_y1, max_x1, max_y1),
    //     vec![e4.clone()],
    //     "purple",
    // );

    // println!(
    //     "e1: {}",
    //     line_intersects(
    //         edges,
    //         corners,
    //         &mut e1.clone().into_iter(),
    //         minimum_x_border,
    //         maximum_x_border,
    //         minimum_y_border,
    //         maximum_y_border,
    //         ordered_corners,
    //         (min_x1, min_y1, max_x1, max_y1),
    //     )
    // );
    // println!(
    //     "e2: {}",
    //     line_intersects(
    //         edges,
    //         corners,
    //         &mut e2.clone().into_iter(),
    //         minimum_x_border,
    //         maximum_x_border,
    //         minimum_y_border,
    //         maximum_y_border,
    //         ordered_corners,
    //         (min_x1, min_y1, max_x1, max_y1),
    //     )
    // );
    // println!(
    //     "e3: {}",
    //     line_intersects(
    //         edges,
    //         corners,
    //         &mut e3.clone().into_iter(),
    //         minimum_x_border,
    //         maximum_x_border,
    //         minimum_y_border,
    //         maximum_y_border,
    //         ordered_corners,
    //         (min_x1, min_y1, max_x1, max_y1),
    //     )
    // );
    // println!(
    //     "e4: {}",
    //     line_intersects(
    //         edges,
    //         corners,
    //         &mut e4.clone().into_iter(),
    //         minimum_x_border,
    //         maximum_x_border,
    //         minimum_y_border,
    //         maximum_y_border,
    //         ordered_corners,
    //         (min_x1, min_y1, max_x1, max_y1),
    //     )
    // );

    return all([e1, e2, e3, e4], |i| {
        line_intersects(
            edges,
            corners,
            &mut i.into_iter(),
            minimum_x_border,
            maximum_x_border,
            minimum_y_border,
            maximum_y_border,
            ordered_corners,
            (min_x1, min_y1, max_x1, max_y1),
        )
    });
    return false;
}

fn is_inside(
    edges: &HashSet<(i64, i64)>,
    corners: &HashSet<(i64, i64)>,
    (x, y): (i64, i64),
    minimum_x_border: i64,
    maximum_x_border: i64,
    minimum_y_border: i64,
    maximum_y_border: i64,
    ordered_corners: &[(i64, i64)],
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
) -> bool {
    // assume p is not an edge
    if edges.contains(&(x, y)) {
        panic!("meow");
    }

    let left = (
        x - minimum_x_border,
        (minimum_x_border - 2, y),
        (-1_i64, 0_i64),
    );
    let right = (
        maximum_x_border - x,
        (maximum_x_border + 2, y),
        (1_i64, 0_i64),
    );
    let top = (
        y - minimum_y_border,
        (x, minimum_y_border - 2),
        (0_i64, -1_i64),
    );
    let bottom = (
        maximum_y_border - y,
        (x, maximum_y_border + 2),
        (0_i64, 1_i64),
    );

    
    let stuff = [left, right, top, bottom]
        .into_iter()
        .sorted_by_key(|f| f.0)
        .next()
        .unwrap();

    // dbg!([left, right, top, bottom]);
    // dbg!(stuff);
    // println!("{x} {y}");
    // println!("- {minimum_x_border} {minimum_y_border}");
    // println!("- {maximum_x_border} {maximum_y_border}");

    // test up
    let mut intersections: usize = 0;
    let (mut nx, mut ny) = (x, y);
    let mut inside_edge = false;
    while (nx, ny) != stuff.1 {
        if edges.contains(&(nx, ny)) {
            if !inside_edge {
                inside_edge = true;
                intersections += 1;
            }
        } else {
            inside_edge = false;
        }
        nx += stuff.2.0;
        ny += stuff.2.1;
        // println!("{} {} bbb {} {} {} {}",stuff.1.0, stuff.1.1, nx, ny, edges.contains(&(nx, ny)), inside_edge);
        // sleep(Duration::from_nanos(100));
    }

    let mut color = "green";
    if intersections.is_multiple_of(2) {
        color = "red";
    }
    // println!("{}-{}: {} intersections", x, y, intersections);
    // visualize_with_points(
    //     &format!("aa/{}-{}xy.svg", x,y),
    //     ordered_corners,
    //     (min_x, min_y, max_x, max_y),
    //     vec![
    //         vec![(x + 1, y), (x - 1, y)],
    //         vec![(x, y - 1), (x, y + 1)],
    //         vec![(x, y), (nx, ny)],
    //     ],
    //     color,
    // );
    // visualize_with_points(
    //     &format!("aa/{}y.svg", y),
    //     ordered_corners,
    //     (min_x, min_y, max_x, max_y),
    //     vec![(0, 0), (x, y), (x, ny)],
    //     color,
    // );
    return !intersections.is_multiple_of(2);
}

fn line_intersects(
    edges: &HashSet<(i64, i64)>,
    corners: &HashSet<(i64, i64)>,
    line: &mut impl Iterator<Item = (i64, i64)>,
    minimum_x_border: i64,
    maximum_x_border: i64,
    minimum_y_border: i64,
    maximum_y_border: i64,
    ordered_corners: &[(i64, i64)],
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
) -> bool {
    let mut intersecting = true;

    // whenever exiting a edge, test if inside or outside
    for p in line {
        if !edges.contains(&p) {
            if intersecting {
                intersecting = false;
                // println!("beam yay");
                if !is_inside(
                    edges,
                    corners,
                    p,
                    minimum_x_border,
                    maximum_x_border,
                    minimum_y_border,
                    maximum_y_border,
                    ordered_corners,
                    (min_x, min_y, max_x, max_y),
                ) {
                    return false;
                }
            }
        } else {
            intersecting = true;
        }
    }

    return true;
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
            for x in min(x1, x2)..=max(x1, x2) {
                edges.insert((x, y1));
            }
        }
        if y1 != y2 {
            for y in min(y1, y2)..=max(y1, y2) {
                edges.insert((x1, y));
            }
        }
    }
    // println!("{} {} {} {}", min_x, min_y, max_x, max_y);
    // println!("{}", (max_x - min_x) * (max_y - min_y));

    // visualize(&corners, (min_x, min_y, max_x, max_y)).unwrap();

    let pairs_sorted = corners
        .iter()
        .tuple_combinations() // iterate over combinations
        .map(|(p, q)| (((p.0 - q.0).abs() + 1) * ((p.1 - q.1).abs() + 1), p, q))
        .sorted_by_key(|v| v.0)
        .rev();

    let corner_hash: HashSet<(i64, i64)> = corners.clone().into_iter().collect();
    println!("{} pairs", pairs_sorted.len());
    for (i, (d, p, q)) in pairs_sorted.enumerate() {
        // println!("{},{} {},{}", p.0, p.1, q.0, q.1);
        println!("{}\t{}", d, i);
        visualize_with_rect(
            &format!("aa/meow.svg"),
            &corners,
            (min_x, min_y, max_x, max_y),
            *p,
            *q,
            "green",
        );

        if rectangle_valid(
            &edges,
            &corner_hash,
            *p,
            *q,
            min_x,
            max_x,
            min_y,
            max_y,
            &corners,
            (min_x, min_y, max_x, max_y),
        ) {
            return d;
        }
        // return 0;
    }

    panic!("meowwwww");
}

fn visualize(
    ordered_corners: &[(i64, i64)],
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
) -> Result<(), std::io::Error> {
    let mut meow = ordered_corners.iter();
    let mut data = Data::new().move_to(*meow.next().unwrap());
    for meowmeow in meow {
        data = data.line_to(*meowmeow);
    }
    data = data.close();

    let mut stroke_width = 100;
    if cfg!(test) {
        stroke_width = 1;
    }

    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "pink")
        .set("stroke-width", stroke_width)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (min_x - 1, min_y - 1, max_x + 1, max_y + 1))
        .add(path);

    svg::save("meow.svg", &document)
}

fn visualize_with_rect(
    name: &str,
    ordered_corners: &[(i64, i64)],
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
    (x1, y1): (i64, i64),
    (x2, y2): (i64, i64),
    color: &str,
) -> Result<(), std::io::Error> {
    let mut meow = ordered_corners.iter();
    let mut data = Data::new().move_to(*meow.next().unwrap());
    for meowmeow in meow {
        data = data.line_to(*meowmeow);
    }
    data = data.close();

    let mut stroke_width = 100;
    if cfg!(test) {
        stroke_width = 1;
    }

    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "pink")
        .set("stroke-width", stroke_width)
        .set("d", data);

    let p1 = (min(x1, x2), min(y1, y2));
    let p2 = (min(x1, x2), max(y1, y2));
    let p3 = (max(x1, x2), max(y1, y2));
    let p4 = (max(x1, x2), min(y1, y2));
    let rect_data = Data::new()
        .move_to(p1)
        .line_to(p2)
        .line_to(p3)
        .line_to(p4)
        .line_to(p1);
    let rect_path = Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", stroke_width)
        .set("d", rect_data);

    let document = Document::new()
        .set("viewBox", (min_x - 1, min_y - 1, max_x + 1, max_y + 1))
        .add(path)
        .add(rect_path);

    svg::save(name, &document)
}

fn visualize_with_points(
    name: &str,
    ordered_corners: &[(i64, i64)],
    (min_x, min_y, max_x, max_y): (i64, i64, i64, i64),
    points: Vec<Vec<(i64, i64)>>,
    color: &str,
) -> Result<(), std::io::Error> {
    let mut meow = ordered_corners.iter();
    let mut data = Data::new().move_to(*meow.next().unwrap());
    for meowmeow in meow {
        data = data.line_to(*meowmeow);
    }
    data = data.close();

    let mut stroke_width = 100;
    if cfg!(test) {
        stroke_width = 1;
    }

    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "pink")
        .set("stroke-width", stroke_width)
        .set("d", data);

    let mut rect_data = Data::new();
    let mut points_cpy: Vec<Vec<(i64, i64)>> = points.clone();
    points_cpy.rotate_left(1);
    for meowmeowmeow in points_cpy.iter() {
        rect_data = rect_data.move_to(meowmeowmeow[0]);
        for meowmeow in meowmeowmeow.iter() {
            rect_data = rect_data.line_to(*meowmeow);
        }
    }
    rect_data = rect_data.close();

    let rect_path = Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("stroke-width", stroke_width)
        .set("d", rect_data);

    let document = Document::new()
        .set("viewBox", (min_x - 1, min_y - 1, max_x + 1, max_y + 1))
        .add(path)
        .add(rect_path);

    svg::save(name, &document)
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
