use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use aoc_runner_derive::aoc;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node([char; 2]);

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.iter().join("")))
    }
}

impl Display for Node {
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
    fn node_groups(&self, n: &Node, adjacent_set: FxHashSet<Node>) -> Option<FxHashSet<Node>> {
        // println!("{:?} {:?}", n, adjacent_set);
        let neigh = FxHashSet::from_iter(self.edges.get(n).unwrap().iter().map(|n| *n));
        let intersection = FxHashSet::from_iter(adjacent_set.intersection(&neigh).map(|n| *n));
        if intersection != adjacent_set {
            return Some(adjacent_set);
        }
        let mut next_adj_set = adjacent_set.clone();
        next_adj_set.insert(*n);
        neigh
            .iter()
            .filter(|neigh_n| !next_adj_set.contains(neigh_n))
            .filter_map(|neigh_n| self.node_groups(neigh_n, next_adj_set.clone()))
            .max_by(|a, b| a.len().cmp(&b.len()))
    }
    fn born_kerbosch(&self, r: FxHashSet<Node>, mut p: FxHashSet<Node>, mut x: FxHashSet<Node>) -> FxHashSet<Node> {
        if p.is_empty() && r.is_empty() {
            return r;
        }
        let p_iter = p.clone(); // se we can modify p
        for node in &p_iter {
            let mut new_r = r.clone();
            new_r.insert(*node);
            let new_p = FxHashSet::from_iter(
                p.intersection(&FxHashSet::from_iter(self.edges.get(&node).unwrap().iter().map(|n| *n)))
                    .map(|n| *n),
            );
            let new_x = FxHashSet::from_iter(
                x.intersection(&FxHashSet::from_iter(self.edges.get(&node).unwrap().iter().map(|n| *n)))
                    .map(|n| *n),
            );

            self.born_kerbosch(new_r, new_p, new_x);
            p.remove(&node);
            x.insert(*node);
        }
        r
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
    // println!("edges: {:?}", network.edges);
    let sets = network.groups_3();
    let t_count = sets.iter().filter(|set| set.iter().any(|s| s.0[0] == 't')).count();
    // println!("groups: {:?}", sets);

    t_count as i64
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let network = parse(input);
    let mut best = FxHashSet::default();
    let mut node_queue = network.nodes.clone();
    let progress = ProgressBar::new(network.nodes.len() as u64);
    while let Some(node) = node_queue.pop() {
        progress.inc(1);
        let net = network.node_groups(&node, FxHashSet::default()).unwrap();
        println!("NODE {} best: {:?}", node, net);
        for checked in &net {
            if let Some(idx) = node_queue.iter().position(|to_remove| to_remove == checked) {
                node_queue.remove(idx);
                progress.inc(1);
            }
        }
        if net.len() > best.len() {
            best = net;
        }
    }
    // println!("{:?}", network.node_groups(&Node(['k', 'a']), FxHashSet::default()));
    println!("best: {:?}", best);
    String::new()
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
