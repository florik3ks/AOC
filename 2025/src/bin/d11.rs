use std::collections::{HashMap, HashSet, VecDeque};
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

#[derive(Clone)]
struct Node {
    outgoing: Vec<usize>,
    incoming: usize,
}

pub fn p2(input: &str) -> i64 {
    let mut node_keys: HashMap<&str, usize> = HashMap::new();
    for (i, l) in input.lines().enumerate() {
        let key = l.split(":").next().unwrap();
        node_keys.insert(key, i);
    }

    let out_index = node_keys.len();
    node_keys.insert("out", out_index);

    let mut nodes = vec![
        Node {
            outgoing: Vec::new(),
            incoming: 0
        };
        node_keys.len()
    ];
    for l in input.lines() {
        let mut split = l.split(":");
        let key = split.next().unwrap(); // ignore key
        for node in split.next().unwrap().trim().split_ascii_whitespace() {
            // add outgoing nodes
            nodes[*node_keys.get(key).unwrap()]
                .outgoing
                .push(*node_keys.get(node).unwrap());
            // increment incoming nodes
            nodes[*node_keys.get(node).unwrap()].incoming += 1;
        }
    }

    let svr_to_fft = get_paths_from_to(
        *node_keys.get("svr").unwrap(),
        *node_keys.get("svr").unwrap(),
        1,
        *node_keys.get("fft").unwrap(),
        &node_keys,
        &nodes,
    );
    let fft_to_dac = get_paths_from_to(
        *node_keys.get("svr").unwrap(),
        *node_keys.get("fft").unwrap(),
        svr_to_fft,
        *node_keys.get("dac").unwrap(),
        &node_keys,
        &nodes,
    );
    let dac_to_out = get_paths_from_to(
        *node_keys.get("svr").unwrap(),
        *node_keys.get("dac").unwrap(),
        fft_to_dac,
        *node_keys.get("out").unwrap(),
        &node_keys,
        &nodes,
    );

    return dac_to_out;
}

#[derive(Clone)]
struct WorkingNode {
    incoming_nodes: usize,
    path_multiplier: i64,
}

fn get_paths_from_to(
    graph_source: usize,
    from: usize,
    from_multiplier: i64,
    to: usize,
    node_keys: &HashMap<&str, usize>,
    nodes: &[Node],
) -> i64 {
    // initialize node data
    let mut node_data = vec![
        WorkingNode {
            incoming_nodes: 0,
            path_multiplier: 0
        };
        node_keys.len()
    ];
    node_data[from].path_multiplier = from_multiplier;

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(graph_source);
    let mut computed: HashSet<usize> = HashSet::new();
    computed.insert(graph_source);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        // only handle node if all incoming nodes have already been handled
        if node_data[current].incoming_nodes != nodes[current].incoming {
            queue.push_back(current);
            continue;
        }
        // if current node is target node, return result
        if current == to {
            return node_data[to].path_multiplier;
        }
        for node in nodes[current].outgoing.iter() {
            // add node to queue if not already in it
            if !computed.contains(node) {
                queue.push_back(*node);
                computed.insert(*node);
            }
            // update multiplier and incoming counter on target node
            node_data[*node].path_multiplier += node_data[current].path_multiplier;
            node_data[*node].incoming_nodes += 1;
        }
    }

    panic!("target node not found");
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
        let expected = 2;
        let example = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
        assert_eq!(p2(example), expected);
    }
}
