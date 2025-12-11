use std::collections::{HashMap, VecDeque};
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

pub fn p1(input: &str) -> i32 {
    let mut node_keys: HashMap<&str, usize> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let key = l.split(":").next().unwrap();
        node_keys.insert(key, i);
    }

    let out_index = node_keys.len();
    node_keys.insert("out", out_index);

    let mut nodes = vec![Vec::<usize>::new(); node_keys.len()];
    for l in input.lines() {
        let mut split = l.split(":");
        let key = split.next().unwrap(); // ignore key
        for node in split.next().unwrap().trim().split_ascii_whitespace() {
            nodes[*node_keys.get(key).unwrap()].push(*node_keys.get(node).unwrap());
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(*node_keys.get("you").unwrap());

    let mut paths = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for node in nodes[current].iter() {
            if *node == out_index {
                paths += 1;
            } else {
                queue.push_back(*node);
            }
        }
    }

    return paths;
}

pub fn p2(input: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 5;
        let example = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
        assert_eq!(p1(example), expected);
    }

    #[test]
    fn test_part2() {
        let expected = 0;
        let example = r"";
        assert_eq!(p2(example), expected);
    }
}
