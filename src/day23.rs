use std::fmt::Debug;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node([char; 2]);

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.iter().join("")))
    }
}

impl From<[char; 2]> for Node {
    fn from(value: [char; 2]) -> Self {
        Node(value)
    }
}

impl TryFrom<Vec<char>> for Node {
    type Error = <[char; 2] as TryFrom<Vec<char>>>::Error;
    fn try_from(value: Vec<char>) -> Result<Self, Self::Error> {
        let array: [char; 2] = value.try_into()?;
        Ok(Self(array))
    }
}

struct Network {
    nodes: Vec<Node>,
    edges: FxHashMap<Node, Vec<Node>>,
}

impl Network {
    fn groups_3(&self) -> FxHashSet<Vec<Node>> {
        let mut sets = FxHashSet::default();
        for n in &self.nodes {
            let neighbours = self.edges.get(n).unwrap();
            for neigh in neighbours {
                let neighbours2 = self.edges.get(neigh).unwrap();
                for neigh2 in neighbours2 {
                    let neighbours3 = self.edges.get(neigh2).unwrap();
                    if neighbours3.contains(n) {
                        let mut set = vec![*n, *neigh, *neigh2];
                        set.sort();
                        sets.insert(set);
                    }
                }
            }
        }
        sets
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let mut nodes = Vec::new();
        let mut edges = FxHashMap::default();
        for line in input.lines() {
            let (node1, node2) = line.split_once('-').unwrap();
            let (node1, node2): (Node, Node) = (
                node1.chars().collect_vec().try_into().unwrap(),
                node2.chars().collect_vec().try_into().unwrap(),
            );
            if !nodes.contains(&node1) {
                nodes.push(node1);
            }
            if !nodes.contains(&node2) {
                nodes.push(node2);
            }
            edges.entry(node1).or_insert(Vec::new()).push(node2);
            edges.entry(node2).or_insert(Vec::new()).push(node1);
        }
        Self { nodes, edges }
    }
}

fn parse(input: &str) -> Network {
    input.into()
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let network = parse(input);
    println!("edges: {:?}", network.edges);
    let sets = network.groups_3();
    let t_count = sets.iter().filter(|set| set.iter().any(|s| s.0[0] == 't')).count();
    println!("groups: {:?}", sets);
    
    t_count as i64
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    }
}
