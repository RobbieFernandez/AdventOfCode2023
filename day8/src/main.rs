use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Network {
    node_map: HashMap<String, (String, String)>,
}

#[derive(Debug)]
struct Node<'a> {
    value: &'a str,
    network: &'a Network,
}

#[derive(Debug)]
struct Directions {
    directions: Vec<Direction>,
}

#[derive(Debug)]
struct DirectionIterator<'a> {
    index: usize,
    directions: &'a Directions,
}

impl From<&char> for Direction {
    fn from(value: &char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid character {}", value),
        }
    }
}

impl Directions {
    fn from_string(input: &str) -> Self {
        let mut directions: Vec<Direction> = Vec::new();
        for c in input.chars() {
            directions.push(Direction::from(&c));
        }

        Self { directions }
    }

    fn len(&self) -> usize {
        self.directions.len()
    }

    fn get(&self, index: usize) -> Direction {
        self.directions[index]
    }

    fn iter(&self) -> DirectionIterator<'_> {
        DirectionIterator {
            index: 0,
            directions: self,
        }
    }
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.directions.len();
        let val = Some(self.directions.get(self.index));
        self.index = (self.index + 1) % length;

        val
    }
}

impl Network {
    fn get_node<'a>(&'a self, value: &'a str) -> Node<'a> {
        Node {
            value,
            network: self,
        }
    }
}

impl<'a> Node<'a> {
    fn get_neighbour(&self, direction: Direction) -> Self {
        let (left, right) = self
            .network
            .node_map
            .get(self.value)
            .expect("node should exist");

        match direction {
            Direction::Left => self.network.get_node(left),
            Direction::Right => self.network.get_node(right),
        }
    }
}

fn parse_input(filename: &str) -> (Directions, Network) {
    let input = read_to_string(filename).expect("Could not read file");
    let mut lines = input.lines();

    let directions = Directions::from_string(lines.next().unwrap());

    lines.next();

    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let mut line = line.split("=");
        let node_value = line.next().unwrap().trim();

        let children = line.next().unwrap().trim();

        let children = &children[1..children.len() - 1];
        let mut children = children.split(',');
        let left = children.next().unwrap().trim();
        let right = children.next().unwrap().trim();

        node_map.insert(node_value.into(), (left.into(), right.into()));
    }

    // Now from the map build the actual tree structure.
    let network = Network { node_map };

    (directions, network)
}

fn count_moves_until_z(directions: &Directions, network: &Network, starting_position: &str) -> u32 {
    let mut direction_iter = directions.iter();

    let mut count = 0;
    let mut current_node = network.get_node(starting_position);

    while !current_node.value.ends_with('Z') {
        current_node = current_node.get_neighbour(direction_iter.next().unwrap());

        count += 1;
    }

    count
}

fn greatest_common_divisor(mut n1: u64, mut n2: u64) -> u64 {
    while n1 != n2 {
        if n1 > n2 {
            n1 -= n2;
        } else {
            n2 -= n1
        }
    }

    n1
}

fn lowest_common_multiple(n1: u64, n2: u64) -> u64 {
    (n1 * n2) / greatest_common_divisor(n1, n2)
}

fn lowest_common_multiple_many(numbers: &[u64]) -> u64 {
    let mut numbers = numbers.to_vec();
    numbers.sort();

    numbers
        .iter()
        .fold(numbers[0], |n1, n2| lowest_common_multiple(n1, *n2))
}

fn solution1(directions: &Directions, network: &Network) -> u32 {
    let mut direction_iter = directions.iter();

    let mut count = 0;
    let mut current_node = network.get_node("AAA");

    while current_node.value != "ZZZ" {
        current_node = current_node.get_neighbour(direction_iter.next().unwrap());

        count += 1;
    }

    count
}

fn solution2(directions: &Directions, network: &Network) -> u64 {
    let starting_node_names = network.node_map.keys().filter(|k| k.ends_with('A'));

    // Solve for each starting position. Then find lowest common multiple of all of them.
    let counts: Vec<u64> = starting_node_names
        .map(|node| count_moves_until_z(directions, network, node) as u64)
        .collect();

    lowest_common_multiple_many(&counts)
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");

    let (directions, network) = parse_input(&filename);
    let answer1 = solution1(&directions, &network);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&directions, &network);
    println!("Solution 2: {}", answer2);
}
