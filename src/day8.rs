use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{fs::File, io::Read};

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Node {
    name: [char; 3],
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        assert_eq!(s.len(), 3);
        Node {
            name: s.chars().collect::<Vec<char>>().try_into().unwrap(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}", self.name[0], self.name[1], self.name[2])
    }
}

fn main() {
    let _input = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    let _input = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    let _input = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";

    let mut input = String::new();
    let _ = File::open("inputs/8.txt")
        .unwrap()
        .read_to_string(&mut input);

    let (route, network) = input.split_once("\n\n").unwrap();
    let mut map: HashMap<Node, (Node, Node)> = HashMap::new();
    for line in network.lines() {
        assert_eq!(line.trim().len(), 16);
        let (src, dst) = line.trim().split_once(' ').unwrap();
        let (_, dst) = dst.split_once('(').unwrap();
        let (left, dst) = dst.split_once(',').unwrap();
        let (right, _) = dst.trim().split_once(')').unwrap();

        let node = Node::from(src);
        let left = Node::from(left);
        let right = Node::from(right);

        map.insert(node, (left, right));
    }

    let mut start_nodes: Vec<Node> = Vec::new();
    for (node, _) in &map {
        if node.name[2] == 'A' {
            start_nodes.push(node.clone());
        }
    }

    let mut nums: Vec<u64> = Vec::new();
    let mut count: u64 = 0;

    for node in start_nodes.iter_mut() {
        loop {
            for direction in route.chars() {
                let node_tmp = map.get(&node).unwrap();
                match direction {
                    'L' => *node = node_tmp.0,
                    'R' => *node = node_tmp.1,
                    _ => unreachable!(),
                }
            }
            if node.name[2] == 'Z' {
                count += route.len() as u64;
                nums.push(count);
                count = 0;
                break;
            } else {
                count += route.len() as u64;
            }
        }
    }

    let mut l = nums[0];
    for num2 in nums.iter().skip(1) {
        l = lcm(l, *num2);
    }
    println!("{l}");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * (b / gcd(a, b));
}
