use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{fs::File, io::Read};

#[derive(Eq, PartialEq, Hash, Debug)]
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

    let mut curr_node = map.get(&Node::from("AAA")).unwrap();
    let mut count: u64 = 0;

    loop {
        for direction in route.chars() {
            match direction {
                'L' => curr_node = map.get(&curr_node.0).unwrap(),
                'R' => curr_node = map.get(&curr_node.1).unwrap(),
                _ => unreachable!(),
            }
        }
        if curr_node != map.get(&Node::from("ZZZ")).unwrap() {
            count += route.len() as u64;
        } else {
            count += route.len() as u64;
            break;
        }
    }

    println!("{count}");
}
