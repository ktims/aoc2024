use aoc_runner_derive::aoc;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::{Debug, Display};

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
        Self(value)
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
    nodes: FxHashSet<Node>,
    edges: FxHashMap<Node, FxHashSet<Node>>,
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
    // Had to study Wikipedia for this one
    // https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn bron_kerbosch(
        &self,
        r: FxHashSet<Node>,
        mut p: FxHashSet<Node>,
        mut x: FxHashSet<Node>,
    ) -> Vec<FxHashSet<Node>> {
        let mut results = Vec::new();
        if p.is_empty() && x.is_empty() {
            return vec![r];
        } else if p.is_empty() {
            return Vec::new();
        }
        // choose the pivot with the most neighbours, to minimize the size of p_iter
        let p_iter = if let Some(pivot) = p.union(&x).max_by(|a, b| self.edges[a].len().cmp(&self.edges[b].len())) {
            FxHashSet::from_iter(p.difference(self.edges.get(pivot).unwrap()).copied())
        } else {
            p.clone()
        };
        for node in &p_iter {
            let mut new_r = r.clone();
            new_r.insert(*node);

            let neighbours = FxHashSet::from_iter(self.edges.get(node).unwrap().iter().copied());
            let new_p = FxHashSet::from_iter(p.intersection(&neighbours).copied());
            let new_x = FxHashSet::from_iter(x.intersection(&neighbours).copied());

            results.extend(self.bron_kerbosch(new_r, new_p, new_x).into_iter());
            p.remove(node);
            x.insert(*node);
        }
        results
    }
    fn maximal_subgraphs(&self) -> Vec<FxHashSet<Node>> {
        self.bron_kerbosch(
            FxHashSet::default(),
            FxHashSet::from_iter(self.nodes.iter().copied()),
            FxHashSet::default(),
        )
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let mut nodes = FxHashSet::default();
        let mut edges = FxHashMap::default();
        for line in input.lines() {
            let (node1, node2) = line.split_once('-').unwrap();
            let (node1, node2): (Node, Node) = (
                node1.chars().collect_vec().try_into().unwrap(),
                node2.chars().collect_vec().try_into().unwrap(),
            );
            if !nodes.contains(&node1) {
                nodes.insert(node1);
            }
            if !nodes.contains(&node2) {
                nodes.insert(node2);
            }
            edges.entry(node1).or_insert(FxHashSet::default()).insert(node2);
            edges.entry(node2).or_insert(FxHashSet::default()).insert(node1);
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

    let sets = network.groups_3();
    let t_count = sets.iter().filter(|set| set.iter().any(|s| s.0[0] == 't')).count();

    t_count as i64
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let network = parse(input);
    let best_sets = network.maximal_subgraphs();
    let largest_set = best_sets.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let mut largest = largest_set.iter().collect_vec();
    largest.sort();
    largest.iter().join(",")
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
